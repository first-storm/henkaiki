use crate::config;
use crate::markdown::MarkdownConverter;
use anyhow::{anyhow, bail, Result};
use dashmap::DashMap;
use lazy_static::lazy_static;
use log::{error, info};
use lru::LruCache;
use serde::Serialize;
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
    sync::{Arc, Mutex},
};

// ===== DATA STRUCTURES =====

/// A unique identifier for articles.
pub type ArticleId = i32;

/// Represents a full article with content in HTML.
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

/// Represents a lighter view of an article (no body content).
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

/// Internal structure describing metadata for an article, as loaded from `metainfo.toml`.
pub struct Metainfo {
    id: i32,
    title: Arc<str>,
    description: Arc<str>,
    markdown_path: Arc<str>,
    date: u32,
    tags: Arc<[String]>,
    keywords: Arc<[String]>,
}

/// Represents whether an article was just fetched from cache or freshly loaded.
#[derive(PartialEq)]
pub enum CachedStatus {
    Cached,
    NotCached,
}

// ===== ARTICLE CACHE =====

/// Manages the LRU cache for recently accessed articles.
struct ArticleCache {
    cache: Arc<Mutex<LruCache<ArticleId, Article>>>,
}

impl ArticleCache {
    fn new(cache: Arc<Mutex<LruCache<ArticleId, Article>>>) -> Self {
        Self { cache }
    }

    fn get(&self, article_id: ArticleId) -> Option<Article> {
        let mut cache = self.cache.lock().unwrap();
        cache.get(&article_id).cloned()
    }

    fn put(&self, article_id: ArticleId, article: Article) {
        let mut cache = self.cache.lock().unwrap();
        cache.put(article_id, article);
    }

    fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }
}

// ===== ARTICLE INDEX =====

/// Holds indices for quick lookups: by article ID, by tag, and a sorted list of IDs.
pub struct ArticleIndex {
    by_id: DashMap<ArticleId, Arc<Metainfo>>,
    by_tag: DashMap<String, Vec<ArticleId>>,
    sorted_ids: Arc<Mutex<Vec<ArticleId>>>,
}

impl ArticleIndex {
    fn new() -> Self {
        Self {
            by_id: DashMap::new(),
            by_tag: DashMap::new(),
            sorted_ids: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn clear(&self) {
        self.by_id.clear();
        self.by_tag.clear();
        let mut locked_ids = self.sorted_ids.lock().unwrap();
        locked_ids.clear();
    }

    fn add_metainfo(&self, metainfo: Arc<Metainfo>) {
        let article_id = metainfo.id;
        self.by_id.insert(article_id, Arc::clone(&metainfo));
        
        // Build inverted index of tag -> article IDs
        for tag in metainfo.tags.iter() {
            self.by_tag.entry(tag.clone()).or_default().push(article_id);
        }
    }

    fn sort_indices(&self) {
        // Sort all IDs globally
        {
            let mut all_ids: Vec<_> = self.by_id.iter().map(|e| *e.key()).collect();
            all_ids.sort_unstable();
            let mut locked_ids = self.sorted_ids.lock().unwrap();
            *locked_ids = all_ids;
        }

        // Sort article IDs within each tag
        for mut entry in self.by_tag.iter_mut() {
            entry.value_mut().sort_unstable();
        }
    }

    fn get_all_ids(&self) -> Vec<ArticleId> {
        let locked_ids = self.sorted_ids.lock().unwrap();
        locked_ids.clone()
    }

    fn get_ids_by_tag(&self, tag: &str) -> Vec<ArticleId> {
        self.by_tag.get(tag).map(|v| v.clone()).unwrap_or_default()
    }

    fn get_metainfo(&self, article_id: ArticleId) -> Option<Arc<Metainfo>> {
        self.by_id.get(&article_id).map(|entry| Arc::clone(entry.value()))
    }
}

// ===== FILE STORAGE =====

/// Handles loading articles from the filesystem
struct ArticleStorage {
    source_dir: PathBuf,
}

impl ArticleStorage {
    fn new(source_dir: PathBuf) -> Self {
        Self { source_dir }
    }

    fn load_article(&self, metainfo: &Metainfo) -> Result<Article> {
        let article_dir = self.source_dir.join(metainfo.id.to_string());
        if !article_dir.exists() || !article_dir.is_dir() {
            bail!("Article directory for ID {} not found", metainfo.id);
        }

        let md_file_path = article_dir.join(&*metainfo.markdown_path);
        if !md_file_path.is_file() {
            bail!(
                "Markdown file '{}' is missing for article ID {}",
                metainfo.markdown_path,
                metainfo.id
            );
        }

        let markdown_content = Self::read_file_as_string(&md_file_path)?;
        // Convert Markdown to HTML if markdown_to_html is enabled in the config
        let content = if config::CONFIG.mainconfig.markdown_to_html {
            markdown_content.to_html_with_config(&config::CONFIG).into()
        } else {
            markdown_content.into()
        };

        Ok(Article {
            id: metainfo.id,
            title: Arc::clone(&metainfo.title),
            description: Arc::clone(&metainfo.description),
            content,
            date: metainfo.date,
            tags: Arc::clone(&metainfo.tags),
            keywords: Arc::clone(&metainfo.keywords),
        })
    }

    fn scan_articles(&self, index: &ArticleIndex) -> Result<()> {
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

            // Parse the TOML file into a Metainfo
            if let Ok(metainfo) = Self::parse_metainfo(&metainfo_path) {
                if metainfo.id != article_id {
                    continue; // skip if mismatch
                }
                let meta_arc = Arc::new(metainfo);
                index.add_metainfo(meta_arc);
            }
        }
        Ok(())
    }

    /// Parse a `metainfo.toml` file from disk.
    fn parse_metainfo(path: &PathBuf) -> Result<Metainfo> {
        let mut file = File::open(path)?;
        let mut toml_content = String::new();
        file.read_to_string(&mut toml_content)?;
        let parsed: toml::Value = toml::from_str(&toml_content)?;

        let article_section = parsed
            .get("article")
            .ok_or_else(|| anyhow!("No [article] section found in {:?}", path))?;

        let tags = Self::parse_string_array(article_section, "tags")?;
        let keywords = Self::parse_string_array(article_section, "keywords")?;

        Ok(Metainfo {
            id: article_section
                .get("id")
                .and_then(|v| v.as_integer())
                .ok_or_else(|| anyhow!("Missing 'id' in {:?}", path))? as i32,
            title: article_section
                .get("title")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing 'title' in {:?}", path))?
                .into(),
            description: article_section
                .get("description")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing 'description' in {:?}", path))?
                .into(),
            markdown_path: article_section
                .get("markdown_path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing 'markdown_path' in {:?}", path))?
                .into(),
            date: article_section
                .get("date")
                .and_then(|v| v.as_integer())
                .ok_or_else(|| anyhow!("Missing 'date' in {:?}", path))? as u32,
            tags: tags.into(),
            keywords: keywords.into(),
        })
    }

    /// Parse an array of strings from a TOML `Value`.
    fn parse_string_array(section: &toml::Value, key: &str) -> Result<Vec<String>> {
        let arr = section
            .get(key)
            .and_then(|v| v.as_array())
            .ok_or_else(|| anyhow!("Missing or invalid '{}' array", key))?;

        arr.iter()
            .map(|v| {
                v.as_str()
                    .ok_or_else(|| anyhow!("Invalid string in '{}'", key))
                    .map(str::to_string)
            })
            .collect()
    }

    /// Read file contents as a UTF-8 string.
    fn read_file_as_string(path: &PathBuf) -> Result<String> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}

// ===== SAMPLE ARTICLE =====

lazy_static! {
    /// A sample article, to be optionally injected based on user config.
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

// ===== PAGINATOR =====

/// Helper for article pagination operations
struct Paginator;

impl Paginator {
    /// Generic pagination helper: given a slice of items, returns the sub-slice for `page_number` or an error if out of range.
    fn paginate<T>(
        data: &[T],
        max_per_page: usize,
        page_number: usize,
    ) -> Result<Option<&[T]>> {
        if max_per_page == 0 {
            // If page size is zero, return nothing
            return Ok(None);
        }
        let total_items = data.len();
        let total_pages = Self::compute_total_pages(total_items, max_per_page);

        // If there's at least one page and page_number is out of range
        if total_pages != 0 && page_number >= total_pages {
            bail!("Page number out of range");
        }

        let start = page_number * max_per_page;
        let end = (start + max_per_page).min(total_items);
        Ok(Some(&data[start..end]))
    }

    /// Compute how many pages are needed to hold `total_items` items with the given `max_per_page`.
    fn compute_total_pages(total_items: usize, max_per_page: usize) -> usize {
        if total_items == 0 || max_per_page == 0 {
            0
        } else {
            (total_items + max_per_page - 1) / max_per_page
        }
    }
}

// ===== MAIN ARTICLES FACADE =====

/// Manages a set of articles from a source directory, plus an LRU cache for recently accessed articles.
pub struct Articles {
    storage: ArticleStorage,
    cache: ArticleCache,
    index: Arc<ArticleIndex>,
}

impl Clone for Articles {
    fn clone(&self) -> Self {
        Articles {
            storage: ArticleStorage::new(self.storage.source_dir.clone()),
            cache: ArticleCache::new(Arc::clone(&self.cache.cache)),
            index: Arc::clone(&self.index),
        }
    }
}

impl Articles {
    /// Create a new `Articles` manager, initializing indices from the filesystem.
    pub fn new(source_dir: PathBuf, cache: Arc<Mutex<LruCache<ArticleId, Article>>>) -> Self {
        info!("Initializing Articles");
        let storage = ArticleStorage::new(source_dir);
        let cache = ArticleCache::new(cache);
        let index = Arc::new(ArticleIndex::new());
        
        let articles = Articles {
            storage,
            cache,
            index,
        };
        if let Err(e) = articles.load_index() {
            error!("Failed to load index: {}", e);
        }
        articles
    }

    /// (Re)loads the entire article index from the filesystem.
    pub fn load_index(&self) -> Result<()> {
        self.index.clear();

        // Optionally insert the sample article
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
            let sample_arc = Arc::new(sample_metainfo);
            self.index.add_metainfo(sample_arc);
        }

        // Walk the source directory for real articles
        self.storage.scan_articles(&self.index)?;

        // Sort indices for efficient access
        self.index.sort_indices();

        Ok(())
    }

    /// Refresh the index by reloading from the filesystem.
    pub fn refresh_index(&self) -> Result<()> {
        self.load_index()
    }

    /// Clear the LRU cache entirely.
    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    /// Attempt to retrieve an article by ID. Returns `(Article, CachedStatus)`.
    pub fn get_article(&self, article_id: ArticleId) -> Result<(Article, CachedStatus)> {
        // If the user requested sample article #0, provide that if configured.
        if article_id == 0 && config::CONFIG.mainconfig.sample_article {
            return Ok((SAMPLE_ARTICLE.clone(), CachedStatus::NotCached));
        }

        // Check the cache first
        if let Some(article) = self.cache.get(article_id) {
            return Ok((article, CachedStatus::Cached));
        }

        // Not in cache, so load from filesystem
        let article = self.load_article_from_filesystem(article_id)?;
        self.cache.put(article_id, article.clone());
        Ok((article, CachedStatus::NotCached))
    }

    /// Helper function to load a single article from disk, converting its Markdown to HTML.
    fn load_article_from_filesystem(&self, article_id: ArticleId) -> Result<Article> {
        let metainfo = self.index.get_metainfo(article_id)
            .ok_or_else(|| anyhow!("Article with ID {} not found", article_id))?;

        // If the user requested sample article #0, provide that if configured
        if article_id == 0 && config::CONFIG.mainconfig.sample_article {
            return Ok(SAMPLE_ARTICLE.clone());
        }

        self.storage.load_article(&metainfo)
    }

    /// Force a refresh of a single article from the filesystem, updating the cache.
    pub fn refresh_article(&self, article_id: ArticleId) -> Result<Article> {
        let article = self.load_article_from_filesystem(article_id)?;
        self.cache.put(article_id, article.clone());
        Ok(article)
    }

    /// Helper method to build a summary from metadata.
    fn build_summary(&self, m: &Metainfo) -> ArticleSummary {
        ArticleSummary {
            id: m.id,
            title: Arc::clone(&m.title),
            description: Arc::clone(&m.description),
            date: m.date,
            tags: Arc::clone(&m.tags),
            keywords: Arc::clone(&m.keywords),
        }
    }

    /// Grab article summaries (by looking up `by_id` index) for the given list of IDs.
    fn get_summaries_from_ids(&self, ids: &[ArticleId]) -> Vec<ArticleSummary> {
        let mut results = Vec::with_capacity(ids.len());
        for &id in ids {
            if let Some(m) = self.index.get_metainfo(id) {
                results.push(self.build_summary(&m));
            }
        }
        results
    }

    // ===== PUBLIC API METHODS =====

    /// Return a list of summaries for all articles (sorted by ID).
    pub fn list_article_summaries(&self) -> Result<Vec<ArticleSummary>> {
        let ids = self.index.get_all_ids();
        Ok(self.get_summaries_from_ids(&ids))
    }

    /// Return a paginated list of summaries for all articles.
    pub fn list_article_summaries_paginated(
        &self,
        max_per_page: usize,
        page_number: usize,
    ) -> Result<Vec<ArticleSummary>> {
        let ids = self.index.get_all_ids();
        let page_slice = match Paginator::paginate(&ids, max_per_page, page_number)? {
            Some(range) => range,
            None => return Ok(vec![]),
        };
        Ok(self.get_summaries_from_ids(page_slice))
    }

    /// Return the number of pages needed given `max_per_page` for *all* articles.
    pub fn get_article_summary_page_count(&self, max_per_page: usize) -> usize {
        let ids = self.index.get_all_ids();
        Paginator::compute_total_pages(ids.len(), max_per_page)
    }

    /// Return all article summaries for a given tag (sorted by ID).
    pub fn list_article_summaries_by_tag(&self, tag: &str) -> Result<Vec<ArticleSummary>> {
        let article_ids = self.index.get_ids_by_tag(tag);
        Ok(self.get_summaries_from_ids(&article_ids))
    }

    /// Return a paginated list of summaries for a given tag.
    pub fn list_article_summaries_by_tag_paginated(
        &self,
        tag: &str,
        max_per_page: usize,
        page_number: usize,
    ) -> Result<Vec<ArticleSummary>> {
        let article_ids = self.index.get_ids_by_tag(tag);
        let page_slice = match Paginator::paginate(&article_ids, max_per_page, page_number)? {
            Some(range) => range,
            None => return Ok(vec![]),
        };
        Ok(self.get_summaries_from_ids(page_slice))
    }

    /// Return the number of pages needed for articles of a given tag.
    pub fn get_article_summary_by_tag_page_count(&self, tag: &str, max_per_page: usize) -> usize {
        let article_ids = self.index.get_ids_by_tag(tag);
        Paginator::compute_total_pages(article_ids.len(), max_per_page)
    }

    /// Search articles by `query` in their title or description, returning all matches sorted by ID.
    pub fn search_articles(&self, query: &str) -> Result<Vec<ArticleSummary>> {
        let ids = self.index.get_all_ids();
        let mut results = Vec::new();
        
        for &id in &ids {
            if let Some(m) = self.index.get_metainfo(id) {
                // Simple substring match
                if m.title.contains(query) || m.description.contains(query) {
                    results.push(self.build_summary(&m));
                }
            }
        }
        
        Ok(results)
    }

    /// Return a paginated list of search results for `query`.
    pub fn search_articles_paginated(
        &self,
        query: &str,
        max_per_page: usize,
        page_number: usize,
    ) -> Result<Vec<ArticleSummary>> {
        let results = self.search_articles(query)?;
        let page_slice = match Paginator::paginate(&results, max_per_page, page_number)? {
            Some(range) => range,
            None => return Ok(vec![]),
        };
        Ok(page_slice.to_vec())
    }

    /// Return the total number of pages for a search result.
    pub fn get_search_article_page_count(&self, query: &str, max_per_page: usize) -> usize {
        match self.search_articles(query) {
            Ok(results) => Paginator::compute_total_pages(results.len(), max_per_page),
            Err(_) => 0,
        }
    }
}
