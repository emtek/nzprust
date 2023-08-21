use anyhow::Result;
use frontend::prs_data_types::{self, Competition, Pilot, Ranking, Root};
use polodb_core::Database;
use scraper::Html;
use serde_json::from_str;
use std::fs;

pub fn add_to_polo(root: &Root) {
    let db = Database::open_file("polostore.db").unwrap();
    let pilot_collection = db.collection::<Pilot>("pilots");
    root.pilots.iter().all(|f| {
        pilot_collection.insert_one(f).unwrap();
        true
    });
    let competition_collection = db.collection::<Competition>("competitions");
    root.competitions.iter().all(|f| {
        competition_collection.insert_one(f).unwrap();
        true
    });
    let ranking_collection = db.collection::<Ranking>("rankings");
    root.rankings.iter().all(|f| {
        ranking_collection.insert_one(f).unwrap();
        true
    });
}

pub fn get_from_polo() {
    let db = Database::open_file("polostore.db").unwrap();
    let collection = db.collection::<Competition>("competitions");
    let pilots = collection.find(None).unwrap();

    for pilot in pilots {
        println!("Competition: {:?}", pilot);
    }
}

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
