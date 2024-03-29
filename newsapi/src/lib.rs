use std::io;

use serde::Deserialize;
use thiserror::Error;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[derive(Error, Debug)]
pub enum NewsApiError {
    #[error("Failed to fetch data form API")]
    RequestFailed(ureq::Error),
    #[error("Failed to parse data")]
    ParseFailed(serde_json::Error),
    #[error("Failed to convert response to string")]
    ResponseToStringFailed(io::Error),
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

pub fn get_articles(url: &String) -> Result<Data, NewsApiError> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| NewsApiError::RequestFailed(e))?
        .into_string()
        .map_err(|e| NewsApiError::ResponseToStringFailed(e))?;

    let articles = serde_json::from_str(&response).map_err(|e| NewsApiError::ParseFailed(e))?;

    Ok(articles)
}
