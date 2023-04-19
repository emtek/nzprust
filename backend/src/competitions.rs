use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::{Months, NaiveDate};
use frontend::prs_data_types::{Competition, Root};
use validator::Validate;

use crate::scoring;

pub fn competition_routes() -> Router<Root> {
    Router::new()
        .route("/api/competitions", get(competitions))
        .route("/api/competition/:id", get(competition))
}

pub fn restricted_competition_routes() -> Router<Root> {
    Router::new().route("/api/competitions", post(create_competition))
}

async fn competitions(State(data): State<Root>) -> Response {
    let mut sorted_competitions = data.competitions.clone();
    sorted_competitions.sort_by(|a, b| b.comp_date.cmp(&a.comp_date));
    Json(&sorted_competitions).into_response()
}

async fn competition(State(data): State<Root>, Path(id): extract::Path<String>) -> Response {
    tracing::info!("Competition {:?} requested", id);
    match data.competitions.iter().find(|c| c.id == id) {
        Some(competition) => Json(competition).into_response(),
        None => (StatusCode::NOT_FOUND).into_response(),
    }
}

async fn create_competition(
    State(data): State<Root>,
    Json(competition): extract::Json<Competition>,
) -> Response {
    let mut rankings = data.rankings.clone();
    match competition.validate() {
        Err(error) => (StatusCode::BAD_REQUEST, Json(error)).into_response(),
        Ok(_) => {
            rankings.sort_by(|a, b| b.date.cmp(&a.date));
            let ranking = rankings.iter().find(|r| {
                let comp_date = competition.comp_date.parse::<NaiveDate>().unwrap();
                let rdate = &&r.date.parse::<NaiveDate>().unwrap();
                let two_years_earlier = comp_date.checked_sub_months(Months::new(24)).unwrap();
                two_years_earlier.lt(&rdate) && (comp_date.gt(&rdate) || comp_date.eq(&rdate))
            });
            match scoring::recalculate_competition(&competition, ranking, &data.competitions) {
                Some(new_competition) => Json(new_competition).into_response(),
                None => (StatusCode::BAD_REQUEST).into_response(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use frontend::prs_data_types::{CompetitionPilot, Placing};
    use serde_json::json;

    use crate::data::data_access::load_data;

    use super::*;

    #[tokio::test]
    async fn competition_should_return_result() {
        let result = competition(
            State(load_data().unwrap()),
            Path("2020-03-01-Rotorua".to_string()),
        )
        .await;
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn competitions_should_return_result() {
        let result = competitions(State(load_data().unwrap())).await;
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_valid_competition_should_return_result() {
        let result = create_competition(
            State(load_data().unwrap()),
            Json(Competition {
                id: "NewComp".to_string(),
                name: "NewComp".to_string(),
                location: "NewLocation".to_string(),
                overseas: false,
                exchange_rate: 1.0,
                comp_date: "2022-01-01".to_string(),
                comp_value: 0.0,
                num_tasks: 2,
                ave_num_participants: 0.0,
                pq: json!(0.0),
                pn: 0.0,
                ta: 0.0,
                td: 0.0,
                placings: vec![Placing {
                    id: 1,
                    pilot: CompetitionPilot {
                        pin: "5410".to_owned(),
                        first_name: "name".to_string(),
                        last_name: "last".to_string(),
                        gender: "male".to_string(),
                    },
                    place: 1,
                    points: 0.0,
                    fai_points: 0.0,
                    pp: 0.0,
                    pplacing: 0.0,
                }],
            }),
        )
        .await;
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_invalid_competition_should_return_badrequest() {
        let result = create_competition(
            State(load_data().unwrap()),
            Json(Competition {
                id: "1".to_string(),
                name: "1".to_string(),
                location: "NewLocation".to_string(),
                overseas: false,
                exchange_rate: 1.0,
                comp_date: "2022".to_string(),
                comp_value: 0.0,
                num_tasks: 2,
                ave_num_participants: 0.0,
                pq: json!(0.0),
                pn: 0.0,
                ta: 0.0,
                td: 0.0,
                placings: vec![Placing {
                    id: 1,
                    pilot: CompetitionPilot {
                        pin: "5410".to_string(),
                        first_name: "name".to_string(),
                        last_name: "last".to_string(),
                        gender: "male".to_string(),
                    },
                    place: 1,
                    points: 0.0,
                    fai_points: 0.0,
                    pp: 0.0,
                    pplacing: 0.0,
                }],
            }),
        )
        .await;
        assert_eq!(result.status(), StatusCode::BAD_REQUEST);
    }
}
