use frontend::prs_data_types;
use std::fs;

use anyhow::Result;
use serde_json::from_str;

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

#[derive(Clone, Debug, PartialEq)]
pub enum MultiError {
    RequestError,
    DeserializeError,
    // etc.
}
