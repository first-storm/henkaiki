use comrak::ComrakOptions;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::{env, fs, path::Path, sync::Arc};

lazy_static! {
    pub static ref CONFIG: Arc<Config> = Arc::new(
        Config::from_file(env::current_dir()
            .map(|mut path| {
                path.push("config.toml");
                if !path.exists() {
                    panic!("Config file not found at {:?}", path);
                }
                path.to_str()
                    .expect("Path contains invalid UTF-8")
                    .to_string()
            })
            .expect("Cannot access current directory"))
        .expect("Failed to load config file")
    );
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub extensions: Extensions,
    pub mainconfig: Main,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            extensions: Extensions::default(),
            mainconfig: Main::default(),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct Main {
    #[serde(default = "default_path")]
    pub articles_dir: String,
    #[serde(default = "default_max_cached_articles")]
    pub max_cached_articles: usize,
    #[serde(default = "default_sample_article")]
    pub sample_article: bool,
    #[serde(default = "default_address")]
    pub address: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_record_cache_stats")]
    pub record_cache_stats: bool,
    #[serde(default = "default_markdown_to_html")]
    pub markdown_to_html: bool,
}

fn default_path() -> String {
    env::current_dir()
        .map(|path| path.join("articles").to_str().unwrap().to_string())
        .unwrap_or_else(|_| panic!("Current directory is not valid UTF-8!"))
}

fn default_max_cached_articles() -> usize { 100 }
fn default_sample_article() -> bool { false }
fn default_address() -> String { "127.0.0.1".to_string() }
fn default_port() -> u16 { 8080 }
fn default_record_cache_stats() -> bool { false }
fn default_markdown_to_html() -> bool { true }

#[derive(Debug, Deserialize, Default)]
pub struct Extensions {
    #[serde(default = "default_true")]
    pub strikethrough: bool,
    #[serde(default = "default_true")]
    pub autolink: bool,
    #[serde(default = "default_true")]
    pub description_lists: bool,
    #[serde(default = "default_true")]
    pub footnotes: bool,
    #[serde(default = "default_front_matter_delimiter")]
    pub front_matter_delimiter: Option<String>,
    #[serde(default = "default_true")]
    pub greentext: bool,
    #[serde(default = "default_header_ids")]
    pub header_ids: Option<String>,
    #[serde(default = "default_true")]
    pub math_code: bool,
    #[serde(default = "default_true")]
    pub math_dollars: bool,
    #[serde(default = "default_true")]
    pub multiline_block_quotes: bool,
    #[serde(default = "default_true")]
    pub shortcodes: bool,
    #[serde(default = "default_true")]
    pub spoiler: bool,
    #[serde(default = "default_true")]
    pub subscript: bool,
    #[serde(default = "default_true")]
    pub superscript: bool,
    #[serde(default = "default_true")]
    pub table: bool,
    #[serde(default = "default_true")]
    pub tagfilter: bool,
    #[serde(default = "default_true")]
    pub tasklist: bool,
    #[serde(default = "default_true")]
    pub underline: bool,
    #[serde(default = "default_true")]
    pub wikilinks_title_after_pipe: bool,
    #[serde(default = "default_true")]
    pub wikilinks_title_before_pipe: bool,
}

fn default_true() -> bool { true }
fn default_front_matter_delimiter() -> Option<String> { None }
fn default_header_ids() -> Option<String> { None }

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn to_comrak_options(&self) -> ComrakOptions {
        let mut options = ComrakOptions::default();
        let ext = &self.extensions;

        options.extension.strikethrough = ext.strikethrough;
        options.extension.autolink = ext.autolink;
        options.extension.description_lists = ext.description_lists;
        options.extension.footnotes = ext.footnotes;
        options.extension.front_matter_delimiter = ext.front_matter_delimiter.clone();
        options.extension.greentext = ext.greentext;
        options.extension.header_ids = ext.header_ids.clone();
        options.extension.math_code = ext.math_code;
        options.extension.math_dollars = ext.math_dollars;
        options.extension.multiline_block_quotes = ext.multiline_block_quotes;
        options.extension.shortcodes = ext.shortcodes;
        options.extension.spoiler = ext.spoiler;
        options.extension.subscript = ext.subscript;
        options.extension.superscript = ext.superscript;
        options.extension.table = ext.table;
        options.extension.tagfilter = ext.tagfilter;
        options.extension.tasklist = ext.tasklist;
        options.extension.underline = ext.underline;
        options.extension.wikilinks_title_after_pipe = ext.wikilinks_title_after_pipe;
        options.extension.wikilinks_title_before_pipe = ext.wikilinks_title_before_pipe;

        options.render.unsafe_ = true;
        options
    }
}
