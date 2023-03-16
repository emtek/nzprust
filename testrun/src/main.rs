use std::{net::SocketAddr, path::PathBuf};

use axum::{
    extract::{self, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, get_service, post},
    Json, Router,
};
use chrono::{Months, NaiveDate};
use data::data_access::*;
use data::highcloud_data_types::HighCloudRoot;
use data::prs_data_types::{Competition, Pilot2, Placing};
use tower_http::services::ServeFile;
use validator::Validate;
mod constants;
mod data;
mod rankings;

async fn pilots() -> Response {
    let root = load_data().unwrap();
    (StatusCode::OK, Json(root.pilots)).into_response()
}

async fn pilot(Path(pin): extract::Path<i64>) -> Response {
    let root = load_data().unwrap();
    if let Some(pilot) = root.pilots.iter().find(|p| p.pin == pin.to_string()) {
        (StatusCode::OK, Json(pilot.clone())).into_response()
    } else {
        (StatusCode::NOT_FOUND).into_response()
    }
}

async fn pilot_competitions(Path(pin): extract::Path<i32>) -> Response {
    let root = load_data().unwrap();
    (
        StatusCode::OK,
        Json(
            &root
                .competitions
                .iter()
                .filter(|c| {
                    c.placings
                        .iter()
                        .any(|placing| placing.pilot.pin == pin.to_string())
                })
                .map(|c| c.clone())
                .collect::<Vec<Competition>>(),
        ),
    )
        .into_response()
}

async fn competitions() -> Response {
    let mut root = load_data().unwrap();
    root.competitions
        .sort_by(|a, b| b.comp_date.cmp(&a.comp_date));
    (StatusCode::OK, Json(&root.competitions)).into_response()
}

async fn competition(Path(id): extract::Path<String>) -> Response {
    let root = load_data().unwrap();

    (
        StatusCode::OK,
        Json(&root.competitions.iter().find(|c| c.id == id).unwrap()),
    )
        .into_response()
}

async fn from_highcloud(Path(comp_id): extract::Path<i32>) -> Response {
    if let Ok(data) = get_data_external::<HighCloudRoot>(format!(
        "http://xc.highcloud.net/get_result.php?comPk={}&_=1678092363685",
        comp_id
    ))
    .await
    {
        let mut tasks = 0;
        if let Some(first) = data.data.first() {
            tasks = first
                .iter()
                .skip(10)
                .take_while(|a| a.is_i64() || a.as_str().unwrap_or("").cmp("").is_ne())
                .count();
        }
        (
            StatusCode::OK,
            Json(&Competition {
                name: data.compinfo.com_name,
                location: data.compinfo.com_location,
                comp_date: data.compinfo.com_date_from,
                num_tasks: tasks as i64,
                placings: data
                    .data
                    .iter()
                    .map(|v| Placing {
                        place: v.get(0).unwrap().as_i64().unwrap(),
                        pilot: Pilot2 {
                            pin: v.get(1).unwrap().as_str().unwrap().to_string(),
                            first_name: v.get(3).unwrap().as_str().unwrap().to_string(),
                            last_name: v.get(3).unwrap().as_str().unwrap().to_string(),
                            gender: v.get(5).unwrap().as_str().unwrap().to_string(),
                        },
                        id: 22,
                        points: 0.0,
                        fai_points: 0.0,
                        pplacing: 0.0,
                        pp: 0.0,
                    })
                    .collect(),
                ..Default::default()
            }),
        )
            .into_response()
    } else {
        (StatusCode::NOT_FOUND).into_response()
    }
}

async fn get_rankings() -> Response {
    let root = load_data().unwrap();

    (StatusCode::OK, Json(root.rankings)).into_response()
}

async fn get_ranking(Path(date): extract::Path<String>) -> Response {
    let root = load_data().unwrap();
    (
        StatusCode::OK,
        Json(
            root.rankings
                .iter()
                .find(|r| r.date.cmp(&date).is_eq())
                .unwrap(),
        ),
    )
        .into_response()
}

async fn create_ranking(Path(date): extract::Path<String>) -> Response {
    let root = load_data().unwrap();
    let date = date.parse::<NaiveDate>();
    match date {
        Ok(date) => {
            let results = rankings::calculate_rankings(&date, &root.competitions);
            match results {
                Some(results) => (StatusCode::OK, Json(results)).into_response(),
                None => (StatusCode::BAD_REQUEST).into_response(),
            }
        }
        Err(_) => (StatusCode::BAD_REQUEST, "Not a valid date").into_response(),
    }
}

async fn create_competition(Json(competition): extract::Json<Competition>) -> Response {
    let mut root = load_data().unwrap();
    let validation_result = competition.validate();
    match validation_result {
        Err(error) => (StatusCode::BAD_REQUEST, Json(error)).into_response(),
        Ok(_) => {
            root.rankings.sort_by(|a, b| b.date.cmp(&a.date));
            let ranking = root.rankings.iter().find(|r| {
                let comp_date = competition.comp_date.parse::<NaiveDate>().unwrap();
                let rdate = &&r.date.parse::<NaiveDate>().unwrap();
                let two_years_earlier = comp_date.checked_sub_months(Months::new(24)).unwrap();
                two_years_earlier.lt(&rdate) && (comp_date.gt(&rdate) || comp_date.eq(&rdate))
            });
            match rankings::recalculate_competition(&competition, ranking, &root.competitions) {
                Some(new_competition) => (StatusCode::OK, Json(new_competition)).into_response(),
                None => (StatusCode::BAD_REQUEST).into_response(),
            }
        }
    }
}

fn setup_server() -> Router {
    let assets_dir = PathBuf::from("./dist");
    let static_files_service = get_service(
        tower_http::services::ServeDir::new(assets_dir)
            .append_index_html_on_directories(true)
            .fallback(ServeFile::new("./dist/index.html")),
    );

    // build our application with a route
    Router::new()
        .fallback(static_files_service)
        .route("/api/competitions", get(competitions))
        .route("/api/competitions", post(create_competition))
        .route("/api/competition/:id", get(competition))
        .route("/api/competition/fromhc/:compid", get(from_highcloud))
        .route("/api/pilots", get(pilots))
        .route("/api/pilot/:pin", get(pilot))
        .route("/api/pilot/:pin/competitions", get(pilot_competitions))
        .route("/api/rankings", get(get_rankings))
        .route("/api/ranking/:date", get(get_ranking))
        .route("/api/rankings/:date", post(create_ranking))
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // run our app with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(setup_server().into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn server_should_be_valid() {
        setup_server();
    }

    #[tokio::test]
    async fn competition_should_return_result() {
        let result = competition(Path("2020-03-01-Rotorua".to_string())).await;
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn from_highcloud_should_return_result() {
        let result = from_highcloud(Path(358)).await;
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn pilots_should_return_result() {
        let result = pilots().await;
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn pilot_should_return_result() {
        let result = pilot(Path(5410)).await;
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn rankings_should_return_result() {
        let result = get_rankings().await;
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn ranking_should_return_result() {
        let result = get_ranking(Path("2019-01-01".to_string())).await;
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_valid_ranking_should_return_result() {
        let result = create_ranking(Path("2022-01-01".to_string())).await;
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_invalid_ranking_should_return_bad_request() {
        let result = create_ranking(Path("2022".to_string())).await;
        assert_eq!(result.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn competitions_should_return_result() {
        let result = competitions().await;
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_valid_competition_should_return_result() {
        let result = create_competition(Json(Competition {
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
                pilot: Pilot2 {
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
        }))
        .await;
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_invalid_competition_should_return_badrequest() {
        let result = create_competition(Json(Competition {
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
                pilot: Pilot2 {
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
        }))
        .await;
        assert_eq!(result.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn pilot_competitions_should_return_result() {
        let result = pilot_competitions(Path(5410)).await;
        assert_eq!(result.status(), StatusCode::OK);
    }
}
