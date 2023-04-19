use anyhow::Result;
use frontend::prs_data_types;
use scraper::Html;
use serde_json::from_str;
use std::fs;

pub fn load_data() -> Result<prs_data_types::Root> {
    let contents: String = fs::read_to_string("./data/nzprsBackup.json")?;
    let r = from_str(&contents)?;
    Ok(r)
}

pub async fn get_data_external<T>(path: String) -> Result<T, MultiError>
where
    T: serde::de::DeserializeOwned,
{
    let response = reqwest::get(path).await;
    match response {
        Err(_) => Err(MultiError::RequestError),
        Ok(response) => match response.text().await {
            Err(_) => Err(MultiError::RequestError),
            Ok(text) => match serde_json::from_str::<T>(&text) {
                Err(_) => Err(MultiError::DeserializeError),
                Ok(result) => Ok(result),
            },
        },
    }
}

pub async fn get_html_external(path: String) -> Result<Html, MultiError> {
    let response = reqwest::get(path).await;
    match response {
        Err(_) => Err(MultiError::RequestError),
        Ok(response) => match response.text().await {
            Err(_) => Err(MultiError::RequestError),
            Ok(text) => Ok(Html::parse_document(&text)),
        },
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MultiError {
    RequestError,
    DeserializeError,
    // etc.
}
