#![allow(non_snake_case)]

mod Error;

use article_scraper::ArticleScraper;
use clap::{arg, command};
use reqwest::Client;
use tokio;
use url::Url;


use Error::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let matches = command!()
        .arg(arg!([url] "The URL to a website to read out").required(true))
        .get_matches();

    let url:&String = matches.get_one::<String>("url").ok_or(AppError::Param("missing url to retrieve".to_string()))?;
    let scraper = ArticleScraper::new(None).await;
    let parsedUrl = Url::parse(url).map_err(|e| AppError::UrlParseError(e))?;
    let client = Client::new();
    let article = scraper.parse(&parsedUrl,false,&client,None).await.map_err(|e| AppError::ScrapeError(e.to_string()))?;
    println!("{}",article.html.unwrap());
    Ok(())
}


