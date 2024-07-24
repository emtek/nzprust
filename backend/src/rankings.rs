use std::{borrow::Borrow, sync::Arc};

use anyhow::Error;
use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use chrono::{NaiveDate, Utc};
use frontend::prs_data_types::{Competition, Ranking, Root};
use surrealdb::{engine::remote::ws::Client, Surreal};
use validator::ValidateLength;

use crate::{competitions, scoring, Record};

pub fn ranking_routes() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/api/rankings", get(get_rankings))
        .route("/api/ranking/:date", get(get_ranking))
        .route("/api/ranking/:date", delete(delete_ranking))
}

async fn get_rankings(State(data): State<Arc<Surreal<Client>>>) -> Response {
    let mut db_response = data.query("select * from rankings").await.unwrap();
    let rankings: Vec<Ranking> = db_response.take(0).unwrap();
    Json(rankings).into_response()
}

async fn get_ranking(
    State(data): State<Arc<Surreal<Client>>>,
    Path(date_input): extract::Path<String>,
) -> Response {
    match date_input.parse::<NaiveDate>() {
        Ok(date) => match date.gt(&Utc::now().date_naive()) {
            true => (StatusCode::NOT_FOUND, "Ranking can't exist yet").into_response(),
            false => {
                let mut db_response = data
                    .query("select * from rankings where internalId = $date")
                    .bind(("date", date))
                    .await
                    .unwrap();
                let rankings: Vec<Ranking> = db_response.take(0).unwrap();
                match rankings.length() {
                    Some(0) => create_ranking(data, date).await,
                    _ => (StatusCode::OK, Json(rankings[0].clone())).into_response(),
                }
            }
        },
        Err(_) => (StatusCode::BAD_REQUEST, "Not a valid date").into_response(),
    }
}

async fn create_ranking(data: Arc<Surreal<Client>>, date: NaiveDate) -> Response {
    let mut db_response = data
        .query("select * from competitions")
        .bind(("date", &date))
        .await
        .unwrap();
    let competitions: Vec<Competition> = db_response.take(0).unwrap();
    let results = scoring::calculate_rankings(&date, &&competitions);
    match results {
        Some(results) => {
            let ranking = Ranking {
                ranking_points: results.clone(),
                date: date.to_string(),
                internal_id: date.to_string(),
            };
            let record: Vec<Record> = data
                .create("rankings")
                .content(ranking.clone())
                .await
                .unwrap();
            Json(ranking.clone()).into_response()
        }
        None => (StatusCode::BAD_REQUEST).into_response(),
    }
}

async fn delete_ranking(
    State(data): State<Arc<Surreal<Client>>>,
    Path(date_input): extract::Path<String>,
) -> Response {
    let db_response = data
        .query("delete rankings where date = $date")
        .bind(("date", &date_input))
        .await;
    match db_response {
        Ok(success) => (StatusCode::OK).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
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
