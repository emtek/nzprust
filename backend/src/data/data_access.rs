use anyhow::Result;
use frontend::prs_data_types::{self, Competition, Pilot, Ranking, Root};

use polodb_core::Database;
use scraper::Html;
use serde::Deserialize;
use serde_json::from_str;
use std::fs;
use surrealdb::engine::remote::ws::{Ws, Wss};
use surrealdb::sql::Thing;
use surrealdb::{Error, Surreal};

pub async fn add_to_surreal(root: &Root) -> Result<bool> {
    // Connect to the server
    let db = Surreal::new::<Wss>("spring-day-hooray-misty-hill-8333.fly.dev").await?;

    // Signin as a namespace, database, or root user
    db.signin(surrealdb::opt::auth::Root {
        username: "nzprsroot",
        password: "nzprsmaxpass123",
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("default").use_db("default").await?;

    // Create a new person with a random id
    // for f in root.pilots.iter() {
    //     let pilots: Vec<Pilot> = db.create("pilots").content(f.clone()).await?;
    // }
    for f in root.competitions.iter() {
        let competitions: Vec<Record> = db.create("competitions").content(f.clone()).await?;
    }
    for f in root.rankings.iter() {
        let rankings: Vec<Record> = db.create("rankings").content(f.clone()).await?;
    }
    // root.competitions.iter().for_each(move |f| {
    //     async move {
    //         let created: &Competition = &db.create("competitions").content(f).await.unwrap();
    //     };
    //     ()
    // });

    // root.rankings.iter().for_each(move |f| {
    //     async move {
    //         let created: &Ranking = &db.create("rankings").content(f).await.unwrap();
    //     };
    //     ()
    // });
    Ok(true)
}

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

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}
