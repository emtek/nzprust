use std::sync::Arc;

use anyhow::Error;
use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::NaiveDate;
use frontend::prs_data_types::{Competition, Ranking, Root};
use polodb_core::Database;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{competitions, scoring};

pub fn ranking_routes() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/api/rankings", get(get_rankings))
        .route("/api/ranking/:date", get(get_ranking))
        .route("/api/rankings/:date", post(create_ranking))
}

async fn get_rankings(State(data): State<Arc<Surreal<Client>>>) -> Response {
    let mut db_response = data.query("select * from rankings").await.unwrap();
    let rankings: Vec<Ranking> = db_response.take(0).unwrap();
    Json(rankings).into_response()
}

async fn get_ranking(
    State(data): State<Arc<Surreal<Client>>>,
    Path(date): extract::Path<String>,
) -> Response {
    let mut db_response = data
        .query("select * from rankings where internalId = $date")
        .bind(("date", date))
        .await
        .unwrap();
    let rankings: Vec<Ranking> = db_response.take(0).unwrap();
    (StatusCode::OK, Json(rankings[0].clone())).into_response()
}

async fn create_ranking(
    State(data): State<Arc<Surreal<Client>>>,
    Path(date): extract::Path<String>,
) -> Response {
    let mut db_response = data
        .query("select * from competitions")
        .bind(("date", &date))
        .await
        .unwrap();
    let competitions: Vec<Competition> = db_response.take(0).unwrap();
    match date.parse::<NaiveDate>() {
        Ok(date) => {
            let results = scoring::calculate_rankings(&date, &&competitions);
            match results {
                Some(results) => Json(results).into_response(),
                None => (StatusCode::BAD_REQUEST).into_response(),
            }
        }
        Err(_) => (StatusCode::BAD_REQUEST, "Not a valid date").into_response(),
    }
}

#[cfg(test)]
mod tests {
    use crate::data::data_access::load_data;

    use super::*;

    // #[tokio::test]
    // async fn rankings_should_return_result() {
    //     let result = get_rankings(State(load_data().unwrap())).await;
    //     assert_eq!(result.status(), StatusCode::OK);
    // }

    // #[tokio::test]
    // async fn ranking_should_return_result() {
    //     let result = get_ranking(State(load_data().unwrap()), Path("2019-01-01".to_string())).await;
    //     assert_eq!(result.status(), StatusCode::OK);
    // }

    // #[tokio::test]
    // async fn create_valid_ranking_should_return_result() {
    //     let result =
    //         create_ranking(State(load_data().unwrap()), Path("2022-01-01".to_string())).await;
    //     assert_eq!(result.status(), StatusCode::OK);
    // }

    // #[tokio::test]
    // async fn create_invalid_ranking_should_return_bad_request() {
    //     let result = create_ranking(State(load_data().unwrap()), Path("2022".to_string())).await;
    //     assert_eq!(result.status(), StatusCode::BAD_REQUEST);
    // }
}
