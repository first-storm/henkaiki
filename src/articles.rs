// articles.rs
use crate::config;
use crate::markdown::MarkdownConverter;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use log::{error, info, warn};
use lru::LruCache;
use serde::Serialize;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    path::PathBuf,
    sync::{Arc, Mutex, RwLock},
};

type ArticleId = i32;

/// Represents an article with its metadata and content.
#[derive(Serialize, Clone)]
pub struct Article {
    pub id: ArticleId,
    pub title: String,
    pub description: String,
    pub content: String,
    pub date: u32,
    pub tags: Vec<String>,
}

/// Represents an article summary without the content.
#[derive(Serialize, Clone)]
pub struct ArticleSummary {
    pub id: ArticleId,
    pub title: String,
    pub description: String,
    pub date: u32,
    pub tags: Vec<String>,
}

/// Helper struct to represent the metainfo.toml contents.
struct Metainfo {
    id: i32,
    title: String,
    description: String,
    markdown_path: String,
    date: u32,
    tags: Vec<String>,
}

/// Manages articles with indexing, caching, and filesystem integration.
pub struct Articles {
    source_dir: PathBuf,
    cache: Arc<Mutex<LruCache<ArticleId, Article>>>,
    index: Arc<RwLock<HashMap<ArticleId, Metainfo>>>,
}

impl Clone for Articles {
    fn clone(&self) -> Self {
        Articles {
            source_dir: self.source_dir.clone(),
            cache: Arc::clone(&self.cache),
            index: Arc::clone(&self.index),
        }
    }
}

// Define the global sample article using lazy_static
lazy_static! {
    static ref SAMPLE_ARTICLE: Article = Article {
        id: 0,
        title: "Universal Declaration of Human Rights".to_string(),
        description: "The Universal Declaration of Human Rights is a seminal document adopted by the United Nations General Assembly on December 10, 1948. This article provides a brief overview of its historical significance, outlining its role in establishing a universal framework for protecting fundamental human rights and freedoms worldwide.".to_string(),
        content: include_str!("udhr.md").to_html_with_config(&config::CONFIG),
        date: 19481210,
        tags: vec!["Politics".to_string(),"History".to_string()],
    };
}

impl Articles {
    /// Initializes a new Articles instance with a shared cache and index.
    ///
    /// # Arguments
    ///
    /// * source_dir - The local directory path where articles are stored.
    /// * cache - The shared LruCache wrapped in Arc and Mutex.
    ///
    /// # Example
    ///
    /// ```rust
    /// let articles = Articles::new(PathBuf::from("/path/to/articles"), shared_cache);
    /// ```
    pub fn new(source_dir: PathBuf, cache: Arc<Mutex<LruCache<ArticleId, Article>>>) -> Self {
        info!(
            "Articles initialized with source directory: {:?}",
            source_dir
        );

        let index = Arc::new(RwLock::new(HashMap::new()));
        let articles = Articles {
            source_dir,
            cache,
            index: Arc::clone(&index),
        };

        // Load the index at initialization
        if let Err(e) = articles.load_index() {
            error!("Failed to load article index: {:?}", e);
        }

        articles
    }

    /// Loads the article index from the filesystem, including all metainfo.
    pub fn load_index(&self) -> Result<()> {
        let mut new_index = HashMap::new();

        // Include the sample article if the flag is set
        if config::CONFIG.mainconfig.sample_article {
            let sample_metainfo = Metainfo {
                id: SAMPLE_ARTICLE.id,
                title: SAMPLE_ARTICLE.title.clone(),
                description: SAMPLE_ARTICLE.description.clone(),
                markdown_path: "udhr.md".to_string(),
                date: SAMPLE_ARTICLE.date,
                tags: SAMPLE_ARTICLE.tags.clone(),
            };
            new_index.insert(SAMPLE_ARTICLE.id, sample_metainfo);
        }

        // Collect all valid article directories and their IDs
        let article_dirs = fs::read_dir(&self.source_dir)
            .with_context(|| format!("Reading articles directory {:?}", self.source_dir))?;

        for entry in article_dirs {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let dir_name = match path.file_name().and_then(|s| s.to_str()) {
                Some(name) => name,
                None => {
                    warn!("Invalid directory name in articles directory: {:?}", path);
                    continue;
                }
            };
            let article_id: ArticleId = match dir_name.parse() {
                Ok(id) => id,
                Err(_) => {
                    warn!("Invalid article directory name: {:?}", dir_name);
                    continue;
                }
            };
            let metainfo_path = path.join("metainfo.toml");
            if !metainfo_path.is_file() {
                warn!("Metainfo file missing for article ID {} in path {:?}", article_id, metainfo_path);
                continue;
            }
            match Self::read_metainfo(&metainfo_path) {
                Ok(metainfo) => {
                    if metainfo.id != article_id {
                        warn!(
                            "ID mismatch in metainfo for article ID {}: metainfo ID is {}",
                            article_id, metainfo.id
                        );
                        continue;
                    }
                    new_index.insert(article_id, metainfo);
                },
                Err(e) => {
                    warn!("Failed to read metainfo for article ID {}: {}", article_id, e);
                    continue;
                }
            }
        }

        let mut index = self.index.write().unwrap();
        *index = new_index;
        info!("Article index loaded with {} entries.", index.len());
        Ok(())
    }

    /// Forces reloading the article index from the filesystem, updating the index.
    pub fn refresh_index(&self) -> Result<()> {
        self.load_index()
    }

    /// Retrieves a summary of all articles without their content, sorted by article ID.
    ///
    /// # Returns
    ///
    /// Returns a vector of ArticleSummary.
    pub fn get_all_articles_without_content(&self) -> Result<Vec<ArticleSummary>> {
        let index = self.index.read().unwrap();
        let mut summaries: Vec<ArticleSummary> = index.values()
            .map(|metainfo| ArticleSummary {
                id: metainfo.id,
                title: metainfo.title.clone(),
                description: metainfo.description.clone(),
                date: metainfo.date,
                tags: metainfo.tags.clone(),
            })
            .collect();

        // Sort summaries by article ID
        summaries.sort_by_key(|s| s.id);

        Ok(summaries)
    }

    /// Retrieves an article by its ID. Loads from filesystem if not cached.
    ///
    /// # Arguments
    ///
    /// * article_id - The ID of the article to retrieve.
    ///
    /// # Returns
    ///
    /// Returns the requested Article if found.
    pub fn get_article(&self, article_id: ArticleId) -> Result<Article> {
        if article_id == 0 && config::CONFIG.mainconfig.sample_article {
            // Return the global sample article
            return Ok(SAMPLE_ARTICLE.clone());
        }

        // Attempt to retrieve the article from the shared cache
        {
            let mut cache = self.cache.lock().unwrap();
            if let Some(article) = cache.get(&article_id) {
                info!("Article ID {} retrieved from cache.", article_id);
                return Ok(article.clone());
            }
        }

        info!(
            "Article ID {} not found in cache. Attempting to load from filesystem.",
            article_id
        );

        // Load the article from the filesystem
        let article = self.load_article_from_fs(article_id)?;

        // Insert the loaded article into the shared cache
        {
            let mut cache = self.cache.lock().unwrap();
            cache.put(article_id, article.clone());
            info!("Article ID {} loaded into cache.", article_id);
        }

        Ok(article)
    }

    /// Forces reloading an article from the filesystem, updating the cache.
    ///
    /// # Arguments
    ///
    /// * article_id - The ID of the article to refresh.
    ///
    /// # Returns
    ///
    /// Returns the refreshed Article.
    pub fn refresh_article(&self, article_id: ArticleId) -> Result<Article> {
        let article = self.load_article_from_fs(article_id)?;
        let mut cache = self.cache.lock().unwrap();
        cache.put(article_id, article.clone());
        info!("Article ID {} refreshed in cache.", article_id);
        Ok(article)
    }

    /// Clears the entire article cache.
    pub fn clear_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
        info!("Article cache cleared.");
    }

    /// Loads an article from the filesystem based on its ID.
    ///
    /// # Arguments
    ///
    /// * article_id - The ID of the article to load.
    ///
    /// # Returns
    ///
    /// Returns the loaded Article.
    fn load_article_from_fs(&self, article_id: ArticleId) -> Result<Article> {
        // Retrieve the metainfo from the index
        let index = self.index.read().unwrap();
        let metainfo = match index.get(&article_id) {
            Some(metainfo) => metainfo,
            None => {
                warn!("Article ID {} not found in index.", article_id);
                anyhow::bail!("Article ID {} not found in index.", article_id);
            }
        };

        // Special handling for sample article
        if article_id == 0 && config::CONFIG.mainconfig.sample_article {
            return Ok(SAMPLE_ARTICLE.clone());
        }

        let article_dir = self.source_dir.join(article_id.to_string());

        // Validate the existence of the article directory
        if !article_dir.exists() || !article_dir.is_dir() {
            warn!(
                "Article directory not found for ID {}: {:?}",
                article_id, article_dir
            );
            anyhow::bail!("Article directory not found for ID {}", article_id);
        }

        let metainfo_path = article_dir.join("metainfo.toml");
        if !metainfo_path.is_file() {
            error!("Metainfo file missing for article ID {}.", article_id);
            anyhow::bail!("Metainfo file missing for article ID {}", article_id);
        }

        // Read the markdown file path from metainfo
        let md_file_path = article_dir.join(&metainfo.markdown_path);
        if !md_file_path.is_file() {
            error!(
                "Markdown file missing for article ID {}: {:?}",
                article_id, md_file_path
            );
            anyhow::bail!(
                "Markdown file missing for article ID {}: {:?}",
                article_id,
                md_file_path
            );
        }

        // Read the markdown content
        let markdown_content = Self::read_file_to_string(&md_file_path).with_context(|| {
            format!(
                "Failed to read markdown content for article ID {}",
                article_id
            )
        })?;

        // Convert markdown to HTML
        let html_content = markdown_content.to_html_with_config(&config::CONFIG);

        Ok(Article {
            id: metainfo.id,
            title: metainfo.title.clone(),
            description: metainfo.description.clone(),
            content: html_content,
            date: metainfo.date,
            tags: metainfo.tags.clone(),
        })
    }

    /// Reads and parses the metainfo.toml file.
    ///
    /// # Arguments
    ///
    /// * path - The path to the metainfo.toml file.
    ///
    /// # Returns
    ///
    /// Returns the parsed Metainfo.
    fn read_metainfo(path: &PathBuf) -> Result<Metainfo> {
        let mut file = File::open(path).with_context(|| format!("Opening {:?}", path))?;
        let mut toml_content = String::new();
        file.read_to_string(&mut toml_content)
            .with_context(|| format!("Reading {:?}", path))?;

        let parsed: toml::Value = toml::from_str(&toml_content)
            .with_context(|| format!("Parsing TOML from {:?}", path))?;

        let article_section = parsed
            .get("article")
            .context("Missing [article] section in metainfo.toml")?;

        // Parse tags as an array of strings
        let tags = article_section
            .get("tags")
            .and_then(|v| v.as_array())
            .context("Missing or invalid 'tags' in metainfo.toml")?
            .iter()
            .map(|v| {
                v.as_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| anyhow::anyhow!("Invalid tag in 'tags'"))
            })
            .collect::<Result<Vec<String>>>()?;

        Ok(Metainfo {
            id: article_section
                .get("id")
                .and_then(|v| v.as_integer())
                .context("Missing or invalid 'id' in metainfo.toml")? as i32,
            title: article_section
                .get("title")
                .and_then(|v| v.as_str())
                .context("Missing or invalid 'title' in metainfo.toml")?
                .to_string(),
            description: article_section
                .get("description")
                .and_then(|v| v.as_str())
                .context("Missing or invalid 'description' in metainfo.toml")?
                .to_string(),
            markdown_path: article_section
                .get("markdown_path")
                .and_then(|v| v.as_str())
                .context("Missing or invalid 'markdown_path' in metainfo.toml")?
                .to_string(),
            date: article_section
                .get("date")
                .and_then(|v| v.as_integer())
                .context("Missing or invalid 'date' in metainfo.toml")? as u32,
            tags,
        })
    }

    /// Reads the entire contents of a file into a string.
    ///
    /// # Arguments
    ///
    /// * path - The path to the file to read.
    ///
    /// # Returns
    ///
    /// Returns the file content as a String.
    fn read_file_to_string(path: &PathBuf) -> Result<String> {
        let mut file = File::open(path).with_context(|| format!("Opening {:?}", path))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .with_context(|| format!("Reading {:?}", path))?;
        Ok(content)
    }
}
