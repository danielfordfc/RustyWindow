// functions defined in lib.rs are importable without mentioning the file name -- news_api::fetch_news instead of news_api::lib::fetch_news
pub mod models;

use std::error::Error;
use models::Articles;

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("HTTP request error")]
    RequestFailed(ureq::Error),
    #[error("JSON parsing error")]
    FailedResponseToString(std::io::Error),
    #[error("Serde JSON error")]
    SerializationError(serde_json::Error),
}

pub fn fetch_news(url: &str) -> Result<Articles, NewsApiError> {
    let body: String = ureq::get(url)
        .call().map_err(|e| NewsApiError::RequestFailed(e))?
        .into_string().map_err(|e | NewsApiError::FailedResponseToString(e))?;

    let items: Articles = serde_json::from_str(&body).map_err(|e |NewsApiError::SerializationError(e))?;
    Ok(items)
}
