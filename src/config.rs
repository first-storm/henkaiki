use comrak::ComrakOptions;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::path::Path;
use std::sync::Arc;
use std::{env, fs};

lazy_static! {
    pub static ref CONFIG: Arc<Config> = Arc::new(
        Config::from_file(match env::current_dir() {
            Ok(mut path) => {
                path.push("config.toml");
                if !path.exists() {
                    panic!("Config file not found at {:?}", path);
                }
                path.to_str()
                    .expect("Path contains invalid UTF-8")
                    .to_string()
            }
            Err(_) => panic!("Cannot access current directory"),
        })
        .expect("Failed to load config file")
    );
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub extensions: Extensions,
    pub mainconfig: Main,
    pub article: ArticleConfig,
}

#[derive(Debug, Deserialize, Default)]
pub struct ArticleConfig {
    #[serde(default = "default_articles_per_page")]
    pub articles_per_page: usize,
}

fn default_articles_per_page() -> usize {
    10 // Default value for articles_per_page
}

#[derive(Debug, Deserialize, Default)]
pub struct Main {
    #[serde(default = "default_path")]
    pub articles_dir: String, // Directory where articles are stored
    #[serde(default = "default_max_cached_articles")]
    pub max_cached_articles: usize, // Maximum number of articles to cache
    #[serde(default = "default_sample_article")]
    pub sample_article: bool, // Flag to include a sample article
    #[serde(default = "default_address")]
    pub address: String, // Server address to bind to
    #[serde(default = "default_port")]
    pub port: u16, // Port number for the server
    #[serde(default = "default_record_cache_stats")]
    pub record_cache_stats: bool, // Whether to record cache hit rate statistics
}

fn default_record_cache_stats() -> bool {
    false // Default to not recording cache statistics
}

fn default_sample_article() -> bool {
    false // Default value for sample_article
}

fn default_path() -> String {
    match env::current_dir() {
        Ok(path) => path.join("articles").to_str().unwrap().to_string(),
        Err(_) => panic!("Current directory is not valid UTF-8!"),
    }
}

fn default_max_cached_articles() -> usize {
    100 // Default max cached articles value
}

fn default_address() -> String {
    "127.0.0.1".to_string() // Default address value
}

fn default_port() -> u16 {
    8080 // Default port value
}

#[derive(Debug, Deserialize)]
pub struct Extensions {
    #[serde(default = "default_true")]
    pub strikethrough: bool,
    #[serde(default = "default_true")]
    pub table: bool,
    #[serde(default = "default_true")]
    pub autolink: bool,
    #[serde(default = "default_true")]
    pub tasklist: bool,
    #[serde(default = "default_true")]
    pub footnotes: bool,
    #[serde(default = "default_true")]
    pub description_lists: bool,
    #[serde(default = "default_true")]
    pub multiline_block_quotes: bool,
    #[serde(default = "default_true")]
    pub math_dollars: bool,
    #[serde(default = "default_true")]
    pub math_code: bool,
    #[serde(default = "default_true")]
    pub wikilinks_title_after_pipe: bool,
    #[serde(default = "default_true")]
    pub wikilinks_title_before_pipe: bool,
    #[serde(default = "default_true")]
    pub spoiler: bool,
    #[serde(default = "default_true")]
    pub greentext: bool,
}

fn default_true() -> bool {
    true
}

impl Default for Extensions {
    fn default() -> Self {
        Self {
            strikethrough: true,
            table: true,
            autolink: true,
            tasklist: true,
            footnotes: true,
            description_lists: true,
            multiline_block_quotes: true,
            math_dollars: true,
            math_code: true,
            wikilinks_title_after_pipe: true,
            wikilinks_title_before_pipe: true,
            spoiler: true,
            greentext: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            extensions: Extensions::default(),
            mainconfig: Default::default(),
            article: Default::default(),
        }
    }
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn to_comrak_options(&self) -> ComrakOptions {
        let mut options = ComrakOptions::default();

        // Set default options
        options.extension.strikethrough = self.extensions.strikethrough;
        options.extension.table = self.extensions.table;
        options.extension.autolink = self.extensions.autolink;
        options.extension.tasklist = self.extensions.tasklist;
        options.extension.footnotes = self.extensions.footnotes;
        options.extension.description_lists = self.extensions.description_lists;
        options.extension.multiline_block_quotes = self.extensions.multiline_block_quotes;
        options.extension.math_dollars = self.extensions.math_dollars;
        options.extension.math_code = self.extensions.math_code;
        options.extension.wikilinks_title_after_pipe = self.extensions.wikilinks_title_after_pipe;
        options.extension.wikilinks_title_before_pipe = self.extensions.wikilinks_title_before_pipe;
        options.extension.spoiler = self.extensions.spoiler;
        options.extension.greentext = self.extensions.greentext;

        // Enable _unsafe by default
        options.render.unsafe_ = true;

        options
    }
}
