use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError{
    #[error("IO error: {0}")]
    IO(#[from] io::Error),
    #[error("HTTP request error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Command line arg error: {0}")]
    Param(String),
}