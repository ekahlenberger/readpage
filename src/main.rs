#![allow(non_snake_case)]

mod Error;

use article_scraper::ArticleScraper;
use clap::{arg, command};
use html2text::from_read;
use html_escape::decode_html_entities;
use reqwest::Client;
use termsize::Size;
use tokio;
use url::Url;


use Error::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError>
{
    let matches = command!()
        .arg(arg!([url] "The URL to a website to read out").required(true))
        .get_matches();

    let url:&String = matches.get_one::<String>("url").ok_or(AppError::Param("missing url to retrieve".to_string()))?;
    let scraper = ArticleScraper::new(None).await;
    let parsedUrl = Url::parse(url).map_err(|e| AppError::UrlParseError(e))?;
    let client = Client::new();
    let article = scraper.parse(&parsedUrl,false,&client,None).await.map_err(|e| AppError::ScrapeError(e.to_string()))?;
    if let Some(html) = article.html
    {
        println!("{}\n", url);
        println!("{}", article.title.unwrap_or_else(|| "No Title".to_string()));

        let readable_text = from_read(
                                      decode_html_entities(&html).as_bytes(),
                                      termsize::get().
                                          unwrap_or(Size { cols: 100, rows: 0 }).
                                          cols as usize);
        println!("{}", readable_text);
    }
    else
    {
        println!("There is nothing readable at: {}", url);
    }
    Ok(())
}