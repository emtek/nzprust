use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::NaiveDate;
use frontend::prs_data_types::Root;

use crate::scoring;

pub fn ranking_routes() -> Router<Root> {
    Router::new()
        .route("/api/rankings", get(get_rankings))
        .route("/api/ranking/:date", get(get_ranking))
        .route("/api/rankings/:date", post(create_ranking))
}

async fn get_rankings(State(data): State<Root>) -> Response {
    (StatusCode::OK, Json(data.rankings)).into_response()
}

async fn get_ranking(State(data): State<Root>, Path(date): extract::Path<String>) -> Response {
    (
        StatusCode::OK,
        Json(
            data.rankings
                .iter()
                .find(|r| r.date.cmp(&date).is_eq())
                .unwrap(),
        ),
    )
        .into_response()
}

async fn create_ranking(State(data): State<Root>, Path(date): extract::Path<String>) -> Response {
    let date = date.parse::<NaiveDate>();
    match date {
        Ok(date) => {
            let results = scoring::calculate_rankings(&date, &data.competitions);
            match results {
                Some(results) => (StatusCode::OK, Json(results)).into_response(),
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

    #[tokio::test]
    async fn rankings_should_return_result() {
        let result = get_rankings(State(load_data().unwrap())).await;
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn ranking_should_return_result() {
        let result = get_ranking(State(load_data().unwrap()), Path("2019-01-01".to_string())).await;
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_valid_ranking_should_return_result() {
        let result =
            create_ranking(State(load_data().unwrap()), Path("2022-01-01".to_string())).await;
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_invalid_ranking_should_return_bad_request() {
        let result = create_ranking(State(load_data().unwrap()), Path("2022".to_string())).await;
        assert_eq!(result.status(), StatusCode::BAD_REQUEST);
    }
}
