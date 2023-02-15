use std::net::SocketAddr;

use axum::{
    body::Body,
    extract::{self, Path},
    http::StatusCode,
    routing::{get, post},
    Form, Json, Router,
};
use chrono::{Months, NaiveDate};
use prs_data_types::{Competition, Ranking};

mod constants;
mod data_access;
mod prs_data_types;
mod rankings;

async fn pilots() -> String {
    let root = data_access::load_data().unwrap();

    format!("{:?}", root.pilots)
}

async fn competitions() -> String {
    let root = data_access::load_data().unwrap();

    format!("{:?}", root.competitions)
}

async fn get_rankings() -> String {
    let root = data_access::load_data().unwrap();

    format!("{:?}", root.rankings)
}

async fn get_ranking(Path(date): extract::Path<String>) -> String {
    let root = data_access::load_data().unwrap();
    format!(
        "{:?}",
        root.rankings.iter().find(|r| r.date.cmp(&date).is_eq())
    )
}

async fn create_ranking(Path(date): extract::Path<String>) -> String {
    let root = data_access::load_data().unwrap();
    let results =
        rankings::calculate_rankings(&date.parse::<NaiveDate>().unwrap(), &root.competitions);
    format!("{:?}", results.unwrap())
}

async fn create_competition(Json(competition): extract::Json<Competition>) -> String {
    let root = data_access::load_data().unwrap();
    let ranking = root.rankings.iter().find(|r| {
        let comp_date = competition.comp_date.parse::<NaiveDate>().unwrap();
        let rdate = &&r.date.parse::<NaiveDate>().unwrap();
        let two_years_earlier = comp_date.checked_sub_months(Months::new(24)).unwrap();
        two_years_earlier.lt(&rdate) && (comp_date.gt(&rdate) || comp_date.eq(&rdate))
    });
    let comp = rankings::recalculate_competition(&competition, ranking, &root.competitions, 1.7);
    format!("{:?}", comp.unwrap())
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/pilots", get(pilots))
        .route("/competitions", get(competitions))
        .route("/competitions", post(create_competition))
        .route("/rankings", get(get_rankings))
        .route("/ranking/:date", get(get_ranking))
        .route("/rankings/:date", post(create_ranking));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
