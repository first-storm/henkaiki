use crate::config;
use crate::markdown::MarkdownConverter;
use anyhow::{Result, bail};
use dashmap::DashMap;
use lazy_static::lazy_static;
use log::info;
use lru::LruCache;
use serde::Serialize;
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub type ArticleId = i32;

#[derive(Clone)]
pub struct Article {
    pub id: ArticleId,
    pub title: Arc<str>,
    pub description: Arc<str>,
    pub content: Arc<str>,
    pub date: u32,
    pub tags: Arc<[String]>,
    pub keywords: Arc<[String]>,
}

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
        state.serialize_field("keywords", self.keywords.as_ref())?;
        state.end()
    }
}

#[derive(Clone)]
pub struct ArticleSummary {
    pub id: ArticleId,
    pub title: Arc<str>,
    pub description: Arc<str>,
    pub date: u32,
    pub tags: Arc<[String]>,
    pub keywords: Arc<[String]>,
}

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
        state.serialize_field("keywords", self.keywords.as_ref())?;
        state.end()
    }
}

pub struct Metainfo {
    id: i32,
    title: Arc<str>,
    description: Arc<str>,
    markdown_path: Arc<str>,
    date: u32,
    tags: Arc<[String]>,
    keywords: Arc<[String]>,
}

pub struct ArticleIndex {
    // Store Metainfo in Arc for shared, read-only access without cloning.
    pub by_id: DashMap<ArticleId, Arc<Metainfo>>,
    pub by_tag: DashMap<String, Vec<ArticleId>>,
    pub sorted_ids: Arc<Mutex<Vec<ArticleId>>>,
}

pub struct Articles {
    source_dir: PathBuf,
    cache: Arc<Mutex<LruCache<ArticleId, Article>>>,
    index: Arc<ArticleIndex>,
}

#[derive(PartialEq)]
pub enum CachedStatus {
    Cached,
    NotCached,
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

lazy_static! {
    static ref SAMPLE_ARTICLE: Article = Article {
        id: 0,
        title: "Universal Declaration of Human Rights".into(),
        description: "The Universal Declaration of Human Rights is a seminal document ...".into(),
        content: include_str!("udhr.md")
            .to_html_with_config(&config::CONFIG)
            .into(),
        date: 19481210,
        tags: vec!["Politics".to_string(), "History".to_string()].into(),
        keywords: vec!["human rights".to_string(), "united nations".to_string()].into(),
    };
}

impl Articles {
    pub fn new(source_dir: PathBuf, cache: Arc<Mutex<LruCache<ArticleId, Article>>>) -> Self {
        info!("Initializing Articles");
        let index = Arc::new(ArticleIndex {
            by_id: DashMap::new(),
            by_tag: DashMap::new(),
            sorted_ids: Arc::new(Mutex::new(Vec::new())),
        });
        let articles = Articles {
            source_dir,
            cache,
            index: Arc::clone(&index),
        };
        if let Err(e) = articles.load_index() {
            log::error!("Failed to load index: {}", e);
        }
        articles
    }

    pub fn load_index(&self) -> Result<()> {
        self.index.by_id.clear();
        self.index.by_tag.clear();

        // Insert sample article if configured
        if config::CONFIG.mainconfig.sample_article {
            let sample_metainfo = Metainfo {
                id: SAMPLE_ARTICLE.id,
                title: SAMPLE_ARTICLE.title.clone(),
                description: SAMPLE_ARTICLE.description.clone(),
                markdown_path: "udhr.md".into(),
                date: SAMPLE_ARTICLE.date,
                tags: SAMPLE_ARTICLE.tags.clone(),
                keywords: SAMPLE_ARTICLE.keywords.clone(),
            };
            let sample_metainfo = Arc::new(sample_metainfo);
            self.index
                .by_id
                .insert(SAMPLE_ARTICLE.id, Arc::clone(&sample_metainfo));
            for tag in sample_metainfo.tags.iter() {
                self.index
                    .by_tag
                    .entry(tag.clone())
                    .or_insert_with(Vec::new)
                    .push(SAMPLE_ARTICLE.id);
            }
        }

        // Iterate through source directory to build the index
        for entry in fs::read_dir(&self.source_dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let dir_name = match path.file_name().and_then(|s| s.to_str()) {
                Some(name) => name,
                None => continue,
            };
            let article_id: ArticleId = match dir_name.parse() {
                Ok(id) => id,
                Err(_) => continue,
            };
            let metainfo_path = path.join("metainfo.toml");
            if !metainfo_path.is_file() {
                continue;
            }
            if let Ok(metainfo) = Self::parse_metainfo(&metainfo_path) {
                if metainfo.id != article_id {
                    continue;
                }
                let meta_arc = Arc::new(metainfo);
                self.index.by_id.insert(article_id, Arc::clone(&meta_arc));
                for tag in meta_arc.tags.iter() {
                    self.index
                        .by_tag
                        .entry(tag.clone())
                        .or_insert_with(Vec::new)
                        .push(article_id);
                }
            }
        }

        // Sort all article IDs
        {
            let mut all_ids: Vec<ArticleId> = self.index.by_id.iter().map(|e| *e.key()).collect();
            all_ids.sort_unstable();
            let mut locked_ids = self.index.sorted_ids.lock().unwrap();
            *locked_ids = all_ids;
        }

        // Sort articles under each tag
        for mut entry in self.index.by_tag.iter_mut() {
            entry.value_mut().sort_unstable();
        }

        Ok(())
    }

    pub fn refresh_index(&self) -> Result<()> {
        self.load_index()
    }

    pub fn clear_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    pub fn get_article(&self, article_id: ArticleId) -> Result<(Article, CachedStatus)> {
        if article_id == 0 && config::CONFIG.mainconfig.sample_article {
            return Ok((SAMPLE_ARTICLE.clone(), CachedStatus::NotCached));
        }

        {
            let mut cache = self.cache.lock().unwrap();
            if let Some(article) = cache.get(&article_id) {
                return Ok((article.clone(), CachedStatus::Cached));
            }
        }

        let article = self.load_article_from_filesystem(article_id)?;
        {
            let mut cache = self.cache.lock().unwrap();
            cache.put(article_id, article.clone());
        }
        Ok((article, CachedStatus::NotCached))
    }

    pub fn refresh_article(&self, article_id: ArticleId) -> Result<Article> {
        let article = self.load_article_from_filesystem(article_id)?;
        let mut cache = self.cache.lock().unwrap();
        cache.put(article_id, article.clone());
        Ok(article)
    }

    pub fn list_article_summaries(&self) -> Result<Vec<ArticleSummary>> {
        let locked_ids = self.index.sorted_ids.lock().unwrap();
        let mut summaries = Vec::with_capacity(locked_ids.len());
        for &id in locked_ids.iter() {
            if let Some(m) = self.index.by_id.get(&id) {
                let m = m.value();
                summaries.push(ArticleSummary {
                    id: m.id,
                    title: Arc::clone(&m.title),
                    description: Arc::clone(&m.description),
                    date: m.date,
                    tags: Arc::clone(&m.tags),
                    keywords: Arc::clone(&m.keywords),
                });
            }
        }
        Ok(summaries)
    }

    pub fn list_article_summaries_paginated(
        &self,
        max_per_page: usize,
        page_number: usize,
    ) -> Result<Vec<ArticleSummary>> {
        let locked_ids = self.index.sorted_ids.lock().unwrap();
        let total_articles = locked_ids.len();

        if max_per_page == 0 {
            return Ok(vec![]);
        }

        let total_pages = (total_articles + max_per_page - 1) / max_per_page;
        if page_number >= total_pages && total_pages != 0 {
            bail!("Page number out of range");
        }

        let start = page_number * max_per_page;
        let end = (start + max_per_page).min(total_articles);
        let page_ids = &locked_ids[start..end];

        let mut summaries = Vec::with_capacity(page_ids.len());
        for &id in page_ids {
            if let Some(m) = self.index.by_id.get(&id) {
                let m = m.value();
                summaries.push(ArticleSummary {
                    id: m.id,
                    title: Arc::clone(&m.title),
                    description: Arc::clone(&m.description),
                    date: m.date,
                    tags: Arc::clone(&m.tags),
                    keywords: Arc::clone(&m.keywords),
                });
            }
        }
        Ok(summaries)
    }

    pub fn get_article_summary_page_count(&self, max_per_page: usize) -> usize {
        let locked_ids = self.index.sorted_ids.lock().unwrap();
        let total_articles = locked_ids.len();
        if total_articles == 0 || max_per_page == 0 {
            return 0;
        }
        (total_articles + max_per_page - 1) / max_per_page
    }

    pub fn list_article_summaries_by_tag(&self, tag: &str) -> Result<Vec<ArticleSummary>> {
        let article_ids = self
            .index
            .by_tag
            .get(tag)
            .map(|v| v.clone())
            .unwrap_or_default();
        let mut summaries = Vec::with_capacity(article_ids.len());
        for article_id in article_ids {
            if let Some(m) = self.index.by_id.get(&article_id) {
                let m = m.value();
                summaries.push(ArticleSummary {
                    id: m.id,
                    title: Arc::clone(&m.title),
                    description: Arc::clone(&m.description),
                    date: m.date,
                    tags: Arc::clone(&m.tags),
                    keywords: Arc::clone(&m.keywords),
                });
            }
        }
        // No need to sort again since article_ids are already sorted at load time.
        Ok(summaries)
    }

    pub fn list_article_summaries_by_tag_paginated(
        &self,
        tag: &str,
        max_per_page: usize,
        page_number: usize,
    ) -> Result<Vec<ArticleSummary>> {
        let article_ids = self
            .index
            .by_tag
            .get(tag)
            .map(|v| v.clone())
            .unwrap_or_default();

        let total_articles = article_ids.len();
        if max_per_page == 0 {
            return Ok(vec![]);
        }
        let total_pages = (total_articles + max_per_page - 1) / max_per_page;

        if page_number >= total_pages && total_pages != 0 {
            bail!("Page number out of range");
        }

        let start = page_number * max_per_page;
        let end = (start + max_per_page).min(total_articles);
        let page_ids = &article_ids[start..end];

        let mut summaries = Vec::with_capacity(page_ids.len());
        for &article_id in page_ids {
            if let Some(m) = self.index.by_id.get(&article_id) {
                let m = m.value();
                summaries.push(ArticleSummary {
                    id: m.id,
                    title: Arc::clone(&m.title),
                    description: Arc::clone(&m.description),
                    date: m.date,
                    tags: Arc::clone(&m.tags),
                    keywords: Arc::clone(&m.keywords),
                });
            }
        }

        Ok(summaries)
    }

    pub fn get_article_summary_by_tag_page_count(&self, tag: &str, max_per_page: usize) -> usize {
        let article_ids = self
            .index
            .by_tag
            .get(tag)
            .map(|v| v.clone())
            .unwrap_or_default();
        if article_ids.is_empty() || max_per_page == 0 {
            return 0;
        }
        let total_articles = article_ids.len();
        (total_articles + max_per_page - 1) / max_per_page
    }

    fn load_article_from_filesystem(&self, article_id: ArticleId) -> Result<Article> {
        let metainfo = match self.index.by_id.get(&article_id) {
            Some(entry) => Arc::clone(entry.value()),
            None => bail!("Article with ID {} not found", article_id),
        };
        if article_id == 0 && config::CONFIG.mainconfig.sample_article {
            return Ok(SAMPLE_ARTICLE.clone());
        }
        let article_dir = self.source_dir.join(article_id.to_string());
        if !article_dir.exists() || !article_dir.is_dir() {
            bail!("Article directory for ID {} not found", article_id);
        }
        let md_file_path = article_dir.join(&*metainfo.markdown_path);
        if !md_file_path.is_file() {
            bail!(
                "Markdown file '{}' is missing for article ID {}",
                metainfo.markdown_path,
                article_id
            );
        }
        let markdown_content = Self::read_file_as_string(&md_file_path)?;
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

    // Search articles by keywords in the title and description
    pub fn search_articles(&self, query: &str) -> Result<Vec<ArticleSummary>> {
        let locked_ids = self.index.sorted_ids.lock().unwrap();
        let mut results = Vec::new();
        for &id in locked_ids.iter() {
            if let Some(m) = self.index.by_id.get(&id) {
                let m = m.value();
                if m.title.contains(query) || m.description.contains(query) {
                    results.push(ArticleSummary {
                        id: m.id,
                        title: Arc::clone(&m.title),
                        description: Arc::clone(&m.description),
                        date: m.date,
                        tags: Arc::clone(&m.tags),
                        keywords: Arc::clone(&m.keywords),
                    });
                }
            }
        }
        Ok(results)
    }

    // paginated search article
    pub fn search_articles_paginated(
        &self,
        query: &str,
        max_per_page: usize,
        page_number: usize,
    ) -> Result<Vec<ArticleSummary>> {
        let results = self.search_articles(query)?;
        let total_articles = results.len();

        if max_per_page == 0 {
            return Ok(vec![]);
        }

        let total_pages = (total_articles + max_per_page - 1) / max_per_page;
        if page_number >= total_pages && total_pages != 0 {
            bail!("Page number out of range");
        }

        let start = page_number * max_per_page;
        let end = (start + max_per_page).min(total_articles);
        Ok(results[start..end].to_vec())
    }

    // total pages for search articles
    pub fn get_search_article_page_count(&self, query: &str, max_per_page: usize) -> usize {
        let results = self.search_articles(query).unwrap_or_default();
        let total_articles = results.len();
        if total_articles == 0 || max_per_page == 0 {
            return 0;
        }
        (total_articles + max_per_page - 1) / max_per_page
    }

    fn parse_metainfo(path: &PathBuf) -> Result<Metainfo> {
        let mut file = File::open(path)?;
        let mut toml_content = String::new();
        file.read_to_string(&mut toml_content)?;
        let parsed: toml::Value = toml::from_str(&toml_content)?;

        let article_section = parsed
            .get("article")
            .ok_or_else(|| anyhow::anyhow!("No [article] section found in {:?}", path))?;
        let tags = Self::parse_string_array(article_section, "tags")?;
        let keywords = Self::parse_string_array(article_section, "keywords")?;

        Ok(Metainfo {
            id: article_section
                .get("id")
                .and_then(|v| v.as_integer())
                .ok_or_else(|| anyhow::anyhow!("Missing 'id' in {:?}", path))?
                as i32,
            title: article_section
                .get("title")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Missing 'title' in {:?}", path))?
                .into(),
            description: article_section
                .get("description")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Missing 'description' in {:?}", path))?
                .into(),
            markdown_path: article_section
                .get("markdown_path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Missing 'markdown_path' in {:?}", path))?
                .into(),
            date: article_section
                .get("date")
                .and_then(|v| v.as_integer())
                .ok_or_else(|| anyhow::anyhow!("Missing 'date' in {:?}", path))?
                as u32,
            tags: tags.into(),
            keywords: keywords.into(),
        })
    }

    fn parse_string_array(section: &toml::Value, key: &str) -> Result<Vec<String>> {
        section
            .get(key)
            .and_then(|v| v.as_array())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid '{}' array", key))?
            .iter()
            .map(|v| {
                v.as_str()
                    .ok_or_else(|| anyhow::anyhow!("Invalid string in '{}'", key))
                    .map(|s| s.to_string())
            })
            .collect()
    }

    fn read_file_as_string(path: &PathBuf) -> Result<String> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}
