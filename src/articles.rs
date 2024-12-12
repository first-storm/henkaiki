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

#[derive(Clone)]
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
    pub by_id: DashMap<ArticleId, Metainfo>,
    pub by_tag: DashMap<String, Vec<ArticleId>>,
}

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
        info!("Articles initialized");
        let index = Arc::new(ArticleIndex {
            by_id: DashMap::new(),
            by_tag: DashMap::new(),
        });
        let articles = Articles {
            source_dir,
            cache,
            index: Arc::clone(&index),
        };
        let _ = articles.load_index();
        articles
    }

    pub fn load_index(&self) -> Result<()> {
        self.index.by_id.clear();
        self.index.by_tag.clear();

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
            for tag in &*sample_metainfo.tags {
                self.index
                    .by_tag
                    .entry(tag.clone())
                    .or_insert_with(Vec::new)
                    .push(SAMPLE_ARTICLE.id);
            }
        }

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
            if let Ok(metainfo) = Self::read_metainfo(&metainfo_path) {
                if metainfo.id != article_id {
                    continue;
                }
                self.index.by_id.insert(article_id, metainfo.clone());
                for tag in &*metainfo.tags {
                    self.index
                        .by_tag
                        .entry(tag.clone())
                        .or_insert_with(Vec::new)
                        .push(article_id);
                }
            }
        }
        Ok(())
    }

    pub fn refresh_index(&self) -> Result<()> {
        self.load_index()
    }

    pub fn get_all_articles_without_content(&self) -> Result<Vec<ArticleSummary>> {
        let mut summaries: Vec<_> = self
            .index
            .by_id
            .iter()
            .map(|entry| {
                let m = entry.value();
                ArticleSummary {
                    id: m.id,
                    title: Arc::clone(&m.title),
                    description: Arc::clone(&m.description),
                    date: m.date,
                    tags: Arc::clone(&m.tags),
                    keywords: Arc::clone(&m.keywords),
                }
            })
            .collect();
        summaries.sort_by_key(|s| s.id);
        Ok(summaries)
    }

    pub fn get_articles_by_tag(&self, tag: &str) -> Result<Vec<ArticleSummary>> {
        let article_ids = self
            .index
            .by_tag
            .get(tag)
            .map(|v| v.clone())
            .unwrap_or_default();
        let mut summaries = Vec::new();
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
        summaries.sort_by_key(|s| s.id);
        Ok(summaries)
    }

    pub fn get_article(&self, article_id: ArticleId) -> Result<Article> {
        if article_id == 0 && config::CONFIG.mainconfig.sample_article {
            return Ok(SAMPLE_ARTICLE.clone());
        }
        {
            let mut cache = self.cache.lock().unwrap();
            if let Some(article) = cache.get(&article_id) {
                return Ok(article.clone());
            }
        }

        let article = self.load_article_from_fs(article_id)?;
        {
            let mut cache = self.cache.lock().unwrap();
            cache.put(article_id, article.clone());
        }
        Ok(article)
    }

    pub fn refresh_article(&self, article_id: ArticleId) -> Result<Article> {
        let article = self.load_article_from_fs(article_id)?;
        let mut cache = self.cache.lock().unwrap();
        cache.put(article_id, article.clone());
        Ok(article)
    }

    pub fn clear_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    fn load_article_from_fs(&self, article_id: ArticleId) -> Result<Article> {
        let metainfo = match self.index.by_id.get(&article_id) {
            Some(entry) => entry.value().clone(),
            None => bail!("Article not found"),
        };
        if article_id == 0 && config::CONFIG.mainconfig.sample_article {
            return Ok(SAMPLE_ARTICLE.clone());
        }
        let article_dir = self.source_dir.join(article_id.to_string());
        if !article_dir.exists() || !article_dir.is_dir() {
            bail!("Article directory not found");
        }
        let md_file_path = article_dir.join(&*metainfo.markdown_path);
        if !md_file_path.is_file() {
            bail!("Markdown file missing");
        }
        let markdown_content = Self::read_file_to_string(&md_file_path)?;
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

    fn read_metainfo(path: &PathBuf) -> Result<Metainfo> {
        let mut file = File::open(path)?;
        let mut toml_content = String::new();
        file.read_to_string(&mut toml_content)?;
        let parsed: toml::Value = toml::from_str(&toml_content)?;

        let article_section = parsed
            .get("article")
            .ok_or_else(|| anyhow::anyhow!("No article section"))?;
        let tags = article_section
            .get("tags")
            .and_then(|v| v.as_array())
            .ok_or_else(|| anyhow::anyhow!("No tags"))?
            .iter()
            .map(|v| {
                v.as_str()
                    .ok_or_else(|| anyhow::anyhow!("Invalid tag"))
                    .map(|s| s.to_string())
            })
            .collect::<Result<Vec<_>>>()?;

        let keywords = article_section
            .get("keywords")
            .and_then(|v| v.as_array())
            .ok_or_else(|| anyhow::anyhow!("No keywords"))?
            .iter()
            .map(|v| {
                v.as_str()
                    .ok_or_else(|| anyhow::anyhow!("Invalid keyword"))
                    .map(|s| s.to_string())
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Metainfo {
            id: article_section
                .get("id")
                .and_then(|v| v.as_integer())
                .ok_or_else(|| anyhow::anyhow!("No id"))? as i32,
            title: article_section
                .get("title")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("No title"))?
                .into(),
            description: article_section
                .get("description")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("No desc"))?
                .into(),
            markdown_path: article_section
                .get("markdown_path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("No markdown"))?
                .into(),
            date: article_section
                .get("date")
                .and_then(|v| v.as_integer())
                .ok_or_else(|| anyhow::anyhow!("No date"))? as u32,
            tags: tags.into(),
            keywords: keywords.into(),
        })
    }

    fn read_file_to_string(path: &PathBuf) -> Result<String> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}
