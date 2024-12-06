use std::io;
use clap::Parser;
use htmd::HtmlToMarkdown;
use scraper::{Html, Selector};

mod render;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    url: String,
}

// Why impl right now?
// For the ability to directly modify options and settings with arguments, no extras required.
impl Args {
    fn get_site_content(&self) -> Result<String, Box<dyn std::error::Error>> {
        let url = &self.url;

        // Default headers
        let res: String = ureq::get(url)
            .set("Example-Header", "header value")
            .call()?
            .into_string()?;

        // Parsing content
        let parsed_content = self.parse_content(res).unwrap();

        Ok(parsed_content)
    }

    fn parse_content(&self, html: String) -> Result<String, Box<dyn std::error::Error>> {


        // Move to outside to increase performance
        let converter = HtmlToMarkdown::builder()
            .skip_tags(vec!["script", "style", "iframe"])
            .build();

        let document = Html::parse_document(&html);
        let selector = Selector::parse("p")?;

        // Performance?
        let mut parsed_document = String::new();
        for element in document.select(&selector) {
            parsed_document.push_str(&element.html());
        }

        let converted_content = converter.convert(&parsed_document)?;

        Ok(converted_content)
    }
}

fn main() -> io::Result<()> {
    // Initialise clap-rs
    let args = Args::parse();

    let content = args.get_site_content().unwrap();

    // Initialise terminal
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let result = render::area(terminal, content);
    ratatui::restore();
    result
}