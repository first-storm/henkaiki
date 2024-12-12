use crate::config;
use crate::markdown::MarkdownConverter;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use log::{error, info, warn};
use lru::LruCache;
use serde::Serialize;
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use dashmap::DashMap;

pub type ArticleId = i32;

/// Represents an article with its metadata and content.
#[derive(Clone)]
pub struct Article {
    pub id: ArticleId,
    pub title: Arc<str>,
    pub description: Arc<str>,
    pub content: Arc<str>,
    pub date: u32,
    pub tags: Arc<[String]>,
    pub keywords: Arc<[String]>, // NEW FIELD
}

// Custom serialization for Article to handle Arc types
impl Serialize for Article {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Article", 7)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("title", self.title.as_ref())?;
        state.serialize_field("description", self.description.as_ref())?;
        state.serialize_field("content", self.content.as_ref())?;
        state.serialize_field("date", &self.date)?;
        state.serialize_field("tags", self.tags.as_ref())?;
        state.serialize_field("keywords", self.keywords.as_ref())?; // SERIALIZE NEW FIELD
        state.end()
    }
}

/// Represents an article summary without the content.
#[derive(Clone)]
pub struct ArticleSummary {
    pub id: ArticleId,
    pub title: Arc<str>,
    pub description: Arc<str>,
    pub date: u32,
    pub tags: Arc<[String]>,
    pub keywords: Arc<[String]>, // NEW FIELD
}

// Custom serialization for ArticleSummary to handle Arc types
impl Serialize for ArticleSummary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("ArticleSummary", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("title", self.title.as_ref())?;
        state.serialize_field("description", self.description.as_ref())?;
        state.serialize_field("date", &self.date)?;
        state.serialize_field("tags", self.tags.as_ref())?;
        state.serialize_field("keywords", self.keywords.as_ref())?; // SERIALIZE NEW FIELD
        state.end()
    }
}

/// Helper struct to represent the metainfo.toml contents.
#[derive(Clone)]
pub struct Metainfo {
    id: i32,
    title: Arc<str>,
    description: Arc<str>,
    markdown_path: Arc<str>,
    date: u32,
    tags: Arc<[String]>,
    keywords: Arc<[String]>, // NEW FIELD
}

/// Structure representing the article index with inverted indices.
pub struct ArticleIndex {
    pub by_id: DashMap<ArticleId, Metainfo>,
    pub by_tag: DashMap<String, Vec<ArticleId>>,
}

/// Manages articles with indexing, caching, and filesystem integration.
pub struct Articles {
    source_dir: PathBuf,
    cache: Arc<Mutex<LruCache<ArticleId, Article>>>,
    index: Arc<ArticleIndex>,
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
        title: "Universal Declaration of Human Rights".into(),
        description: "The Universal Declaration of Human Rights is a seminal document ...".into(),
        content: include_str!("udhr.md").to_html_with_config(&config::CONFIG).into(),
        date: 19481210,
        tags: vec!["Politics".to_string(), "History".to_string()].into(),
        keywords: vec!["human rights".to_string(), "united nations".to_string()].into(), // SAMPLE KEYWORDS
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

        let index = Arc::new(ArticleIndex {
            by_id: DashMap::new(),
            by_tag: DashMap::new(),
        });

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
        self.index.by_id.clear();
        self.index.by_tag.clear();

        // Include the sample article if the flag is set
        if config::CONFIG.mainconfig.sample_article {
            let sample_metainfo = Metainfo {
                id: SAMPLE_ARTICLE.id,
                title: SAMPLE_ARTICLE.title.clone().into(),
                description: SAMPLE_ARTICLE.description.clone().into(),
                markdown_path: "udhr.md".into(),
                date: SAMPLE_ARTICLE.date,
                tags: SAMPLE_ARTICLE.tags.clone().into(),
                keywords: SAMPLE_ARTICLE.keywords.clone().into(),
            };
            self.index
                .by_id
                .insert(SAMPLE_ARTICLE.id, sample_metainfo.clone());

            // Update by_tag
            for tag in &*sample_metainfo.tags {
                self.index
                    .by_tag
                    .entry(tag.clone())
                    .or_insert_with(Vec::new)
                    .push(SAMPLE_ARTICLE.id);
            }
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
                warn!(
                    "Metainfo file missing for article ID {} in path {:?}",
                    article_id, metainfo_path
                );
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
                    self.index.by_id.insert(article_id, metainfo.clone());

                    // Update by_tag
                    for tag in &*metainfo.tags {
                        self.index
                            .by_tag
                            .entry(tag.clone())
                            .or_insert_with(Vec::new)
                            .push(article_id);
                    }
                }
                Err(e) => {
                    warn!("Failed to read metainfo for article ID {}: {}", article_id, e);
                    continue;
                }
            }
        }

        info!(
            "Article index loaded with {} entries.",
            self.index.by_id.len()
        );
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
        let mut summaries: Vec<ArticleSummary> = self
            .index
            .by_id
            .iter()
            .map(|entry| {
                let metainfo = entry.value();
                ArticleSummary {
                    id: metainfo.id,
                    title: Arc::clone(&metainfo.title),
                    description: Arc::clone(&metainfo.description),
                    date: metainfo.date,
                    tags: Arc::clone(&metainfo.tags),
                    keywords: Arc::clone(&metainfo.keywords),
                }
            })
            .collect();

        summaries.sort_by_key(|s| s.id);
        Ok(summaries)
    }

    /// Retrieves a summary of articles by a specific tag without their content.
    ///
    /// # Arguments
    ///
    /// * tag - The tag to filter articles by.
    ///
    /// # Returns
    ///
    /// Returns a vector of ArticleSummary.
    pub fn get_articles_by_tag(&self, tag: &str) -> Result<Vec<ArticleSummary>> {
        let article_ids = match self.index.by_tag.get(tag) {
            Some(entry) => entry.value().clone(),
            None => Vec::new(),
        };

        let mut summaries = Vec::new();
        for article_id in article_ids {
            if let Some(metainfo) = self.index.by_id.get(&article_id) {
                let metainfo = metainfo.value();
                summaries.push(ArticleSummary {
                    id: metainfo.id,
                    title: Arc::clone(&metainfo.title),
                    description: Arc::clone(&metainfo.description),
                    date: metainfo.date,
                    tags: Arc::clone(&metainfo.tags),
                    keywords: Arc::clone(&metainfo.keywords),
                });
            }
        }

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
        let metainfo = match self.index.by_id.get(&article_id) {
            Some(entry) => entry.value().clone(),
            None => anyhow::bail!("Article ID {} not found in index.", article_id),
        };

        // Special handling for sample article
        if article_id == 0 && config::CONFIG.mainconfig.sample_article {
            return Ok(SAMPLE_ARTICLE.clone());
        }

        let article_dir = self.source_dir.join(article_id.to_string());
        if !article_dir.exists() || !article_dir.is_dir() {
            anyhow::bail!("Article directory not found for ID {}", article_id);
        }

        let md_file_path = article_dir.join(&*metainfo.markdown_path);
        if !md_file_path.is_file() {
            anyhow::bail!(
                "Markdown file missing for article ID {}: {:?}",
                article_id,
                md_file_path
            );
        }

        // Read and convert the markdown content
        let markdown_content = Self::read_file_to_string(&md_file_path)
            .with_context(|| format!("Failed to read markdown content for article ID {}", article_id))?;
        let html_content = markdown_content.to_html_with_config(&config::CONFIG);

        Ok(Article {
            id: metainfo.id,
            title: Arc::clone(&metainfo.title),
            description: Arc::clone(&metainfo.description),
            content: html_content.into(),
            date: metainfo.date,
            tags: Arc::clone(&metainfo.tags),
            keywords: Arc::clone(&metainfo.keywords),
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

        // Parse keywords as an array of strings (NEW)
        let keywords = article_section
            .get("keywords")
            .and_then(|v| v.as_array())
            .context("Missing or invalid 'keywords' in metainfo.toml")?
            .iter()
            .map(|v| {
                v.as_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| anyhow::anyhow!("Invalid keyword in 'keywords'"))
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
                .into(),
            description: article_section
                .get("description")
                .and_then(|v| v.as_str())
                .context("Missing or invalid 'description' in metainfo.toml")?
                .into(),
            markdown_path: article_section
                .get("markdown_path")
                .and_then(|v| v.as_str())
                .context("Missing or invalid 'markdown_path' in metainfo.toml")?
                .into(),
            date: article_section
                .get("date")
                .and_then(|v| v.as_integer())
                .context("Missing or invalid 'date' in metainfo.toml")? as u32,
            tags: tags.into(),
            keywords: keywords.into(), // NEW FIELD
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
