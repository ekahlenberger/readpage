#![allow(non_snake_case)]

mod Error;

use clap::{arg, command};
use std::io::{self};
use reqwest::get;


use Error::AppError;

fn main() -> Result<(), AppError> {
    let matches = command!()
        .arg(arg!([url] "The URL to a website to read out").required(true))
        .get_matches();

    // Retrieve the URL argument

    let url:&String = matches.get_one::<String>("url").ok_or(AppError::Param("missing url to retrieve".to_string()))?;

    println!("requesting website content from {}", url);

    let raw_content = retrieve_site_content(url).map_err(|e| AppError::Reqwest(e))?;

    println!("{}",raw_content);
    Ok(())
}

fn retrieve_site_content(url: &str) -> Result<String,reqwest::Error> {

    let mut moddedUrl:String = url.to_string();
    if !url.to_uppercase().starts_with("HTTP://") && !url.to_uppercase().starts_with("HTTPS://"){
        moddedUrl = "https://".to_string() + url;
    }
    return reqwest::blocking::get(moddedUrl)?.text()
}



