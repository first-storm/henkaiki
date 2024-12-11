use comrak::{markdown_to_html, ComrakOptions};
pub use super::config::Config;

pub trait MarkdownConverter {
    fn to_html(&self) -> String;
    fn to_html_with_config(&self, config: &Config) -> String;
    fn to_html_with_options(&self, options: &ComrakOptions) -> String;
}

impl MarkdownConverter for str {
    fn to_html(&self) -> String {
        let config = Config::default();
        self.to_html_with_config(&config)
    }

    fn to_html_with_config(&self, config: &Config) -> String {
        let options = config.to_comrak_options();
        markdown_to_html(self, &options)
    }

    fn to_html_with_options(&self, options: &ComrakOptions) -> String {
        markdown_to_html(self, options)
    }
}

impl MarkdownConverter for String {
    fn to_html(&self) -> String {
        self.as_str().to_html()
    }

    fn to_html_with_config(&self, config: &Config) -> String {
        self.as_str().to_html_with_config(config)
    }

    fn to_html_with_options(&self, options: &ComrakOptions) -> String {
        self.as_str().to_html_with_options(options)
    }
}