use std::sync::Arc;

use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use frontend::prs_data_types::{Competition, Pilot, Root};
use surrealdb::{engine::remote::ws::Client, Surreal};

pub fn pilot_routes() -> Router<Arc<Surreal<Client>>> {
    Router::new()
        .route("/api/pilots", get(pilots))
        .route("/api/pilot/:pin", get(pilot))
        .route("/api/pilot/:pin/competitions", get(pilot_competitions))
}

async fn pilots(State(data): State<Arc<Surreal<Client>>>) -> Response {
    let mut db_response = data.query("select * from pilots").await.unwrap();
    let pilots: Vec<Pilot> = db_response.take(0).unwrap();
    (StatusCode::OK, Json(pilots)).into_response()
}

async fn pilot(
    State(data): State<Arc<Surreal<Client>>>,
    Path(pin): extract::Path<i64>,
) -> Response {
    let mut db_response = data.query("select * from pilots").await.unwrap();
    let pilots: Vec<Pilot> = db_response.take(0).unwrap();
    match pilots.iter().find(|p| p.pin == pin.to_string()) {
        Some(pilot) => (StatusCode::OK, Json(pilot.clone())).into_response(),
        None => (StatusCode::NOT_FOUND).into_response(),
    }
}

async fn pilot_competitions(
    State(data): State<Arc<Surreal<Client>>>,
    Path(pin): extract::Path<i32>,
) -> Response {
    let mut db_response = data
        .query("select * from competitions order by compDate desc")
        .await
        .unwrap();
    let competitions: Vec<Competition> = db_response.take(0).unwrap();
    Json(
        competitions
            .iter()
            .filter(|c| {
                c.placings
                    .iter()
                    .any(|placing| placing.pilot.pin == pin.to_string())
            })
            .map(|c| c.clone())
            .collect::<Vec<Competition>>(),
    )
    .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::data_access::load_data;

    // #[tokio::test]
    // async fn pilot_competitions_should_return_result() {
    //     let result = pilot_competitions(State(load_data().unwrap()), Path(5410)).await;
    //     assert_eq!(result.status(), StatusCode::OK);
    // }

    // #[tokio::test]
    // async fn pilots_should_return_result() {
    //     let result = pilots(State(load_data().unwrap())).await;
    //     assert_eq!(result.status(), StatusCode::OK);
    // }

    // #[tokio::test]
    // async fn pilot_should_return_result() {
    //     let result = pilot(State(load_data().unwrap()), Path(5410)).await;
    //     assert_eq!(result.status(), StatusCode::OK);
    // }
}
