use std::net::SocketAddr;

use axum::{
    body::Body,
    extract::{self, Path},
    http::StatusCode,
    routing::{get, post},
    Form, Json, Router,
};
use chrono::{Months, NaiveDate};
use prs_data_types::{Competition, Pilot, Ranking};
use tower_http::cors::{Any, CorsLayer};

mod constants;
mod data_access;
mod prs_data_types;
mod rankings;

async fn pilots() -> String {
    let root = data_access::load_data().unwrap();

    format!(
        "{}",
        serde_json::to_string::<Vec<Pilot>>(&root.pilots).unwrap()
    )
}

async fn pilot(Path(pin): extract::Path<String>) -> String {
    let root = data_access::load_data().unwrap();

    format!(
        "{}",
        serde_json::to_string::<Pilot>(&root.pilots.iter().find(|p| p.pin == pin).unwrap())
            .unwrap()
    )
}

async fn pilot_competitions(Path(pin): extract::Path<String>) -> String {
    let root = data_access::load_data().unwrap();

    format!(
        "{}",
        serde_json::to_string::<Vec<Competition>>(
            &root
                .competitions
                .iter()
                .filter(|c| c.placings.iter().any(|placing| placing.pilot.pin == pin))
                .map(|c| c.clone())
                .collect()
        )
        .unwrap()
    )
}

async fn competitions() -> String {
    let mut root = data_access::load_data().unwrap();
    root.competitions
        .sort_by(|a, b| b.comp_date.cmp(&a.comp_date));
    format!(
        "{}",
        serde_json::to_string::<Vec<Competition>>(&root.competitions).unwrap()
    )
}

async fn competition(Path(id): extract::Path<String>) -> String {
    let root = data_access::load_data().unwrap();

    format!(
        "{}",
        serde_json::to_string::<Competition>(
            &root.competitions.iter().find(|c| c.id == id).unwrap()
        )
        .unwrap()
    )
}

async fn get_rankings() -> String {
    let root = data_access::load_data().unwrap();

    format!("{:?}", root.rankings)
}

async fn get_ranking(Path(date): extract::Path<String>) -> String {
    let root = data_access::load_data().unwrap();

    format!(
        "{}",
        serde_json::to_string::<Ranking>(
            root.rankings
                .iter()
                .find(|r| r.date.cmp(&date).is_eq())
                .unwrap()
        )
        .unwrap()
    )
}

async fn create_ranking(Path(date): extract::Path<String>) -> String {
    let root = data_access::load_data().unwrap();
    let results =
        rankings::calculate_rankings(&date.parse::<NaiveDate>().unwrap(), &root.competitions);
    format!("{:?}", results.unwrap())
}

async fn create_competition(Json(competition): extract::Json<Competition>) -> String {
    let mut root = data_access::load_data().unwrap();
    root.rankings.sort_by(|a, b| b.date.cmp(&a.date));
    let ranking = root.rankings.iter().find(|r| {
        let comp_date = competition.comp_date.parse::<NaiveDate>().unwrap();
        let rdate = &&r.date.parse::<NaiveDate>().unwrap();
        let two_years_earlier = comp_date.checked_sub_months(Months::new(24)).unwrap();
        two_years_earlier.lt(&rdate) && (comp_date.gt(&rdate) || comp_date.eq(&rdate))
    });
    let comp = rankings::recalculate_competition(&competition, ranking, &root.competitions);
    format!("{:?}", comp.unwrap())
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new().allow_origin(Any);

    // build our application with a route
    let app = Router::new()
        .route("/pilots", get(pilots))
        .route("/pilot/:pin", get(pilot))
        .route("/pilot/:pin/competitions", get(pilot_competitions))
        .route("/competition/:id", get(competition))
        .route("/competitions", get(competitions))
        .route("/competitions", post(create_competition))
        .route("/rankings", get(get_rankings))
        .route("/ranking/:date", get(get_ranking))
        .route("/rankings/:date", post(create_ranking))
        .layer(cors);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
