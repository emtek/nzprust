use std::sync::Arc;

use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use polodb_core::Database;

use frontend::prs_data_types::{Competition, Pilot, Root};

pub fn pilot_routes() -> Router<Arc<Database>> {
    Router::new()
        .route("/api/pilots", get(pilots))
        .route("/api/pilot/:pin", get(pilot))
        .route("/api/pilot/:pin/competitions", get(pilot_competitions))
}

async fn pilots(State(data): State<Arc<Database>>) -> Response {
    let pilots = data
        .collection::<Pilot>("pilots")
        .find(None)
        .unwrap()
        .flatten()
        .collect::<Vec<Pilot>>();
    (StatusCode::OK, Json(pilots)).into_response()
}

async fn pilot(State(data): State<Arc<Database>>, Path(pin): extract::Path<i64>) -> Response {
    let pilots = data
        .collection::<Pilot>("pilots")
        .find(None)
        .unwrap()
        .flatten()
        .collect::<Vec<Pilot>>();
    match pilots.iter().find(|p| p.pin == pin.to_string()) {
        Some(pilot) => (StatusCode::OK, Json(pilot.clone())).into_response(),
        None => (StatusCode::NOT_FOUND).into_response(),
    }
}

async fn pilot_competitions(
    State(data): State<Arc<Database>>,
    Path(pin): extract::Path<i32>,
) -> Response {
    let competitions = data
        .collection::<Competition>("competitions")
        .find(None)
        .unwrap()
        .flatten()
        .collect::<Vec<Competition>>();
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
