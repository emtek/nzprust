use std::{net::SocketAddr, path::PathBuf};

use axum::{
    extract::{self, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, get_service, post},
    Json, Router,
};
use chrono::{Months, NaiveDate};
use data_access::get_data_external;
use highcloud_data_types::HighCloudRoot;
use prs_data_types::{Competition, Pilot2, Placing};
use tower_http::services::ServeFile;
mod constants;
mod data_access;
mod highcloud_data_types;
mod prs_data_types;
mod rankings;

async fn pilots() -> Response {
    let root = data_access::load_data().unwrap();
    (StatusCode::OK, Json(root.pilots)).into_response()
}

async fn pilot(Path(pin): extract::Path<i64>) -> Response {
    let root = data_access::load_data().unwrap();
    if let Some(pilot) = root.pilots.iter().find(|p| p.pin == pin.to_string()) {
        (StatusCode::OK, Json(pilot.clone())).into_response()
    } else {
        (StatusCode::NOT_FOUND).into_response()
    }
}

async fn pilot_competitions(Path(pin): extract::Path<String>) -> Response {
    let root = data_access::load_data().unwrap();
    (
        StatusCode::OK,
        Json(
            &root
                .competitions
                .iter()
                .filter(|c| c.placings.iter().any(|placing| placing.pilot.pin == pin))
                .map(|c| c.clone())
                .collect::<Vec<Competition>>(),
        ),
    )
        .into_response()
}

async fn competitions() -> Response {
    let mut root = data_access::load_data().unwrap();
    root.competitions
        .sort_by(|a, b| b.comp_date.cmp(&a.comp_date));
    (StatusCode::OK, Json(&root.competitions)).into_response()
}

async fn competition(Path(id): extract::Path<String>) -> Response {
    let root = data_access::load_data().unwrap();

    (
        StatusCode::OK,
        Json(&root.competitions.iter().find(|c| c.id == id).unwrap()),
    )
        .into_response()
}

async fn from_highcloud(Path(comp_id): extract::Path<String>) -> Response {
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
        ).into_response()
    } else {
        (StatusCode::NOT_FOUND).into_response()
    }
}

async fn get_rankings() -> Response {
    let root = data_access::load_data().unwrap();

    (StatusCode::OK, Json(root.rankings)).into_response()
}

async fn get_ranking(Path(date): extract::Path<String>) -> Response {
    let root = data_access::load_data().unwrap();

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
    let assets_dir = PathBuf::from("./dist");
    let static_files_service = get_service(
        tower_http::services::ServeDir::new(assets_dir)
            .append_index_html_on_directories(true)
            .fallback(ServeFile::new("./dist/index.html")),
    );

    // build our application with a route
    let app = Router::new()
        .fallback(static_files_service)
        .route("/api/pilots", get(pilots))
        .route("/api/pilot/:pin", get(pilot))
        .route("/api/pilot/:pin/competitions", get(pilot_competitions))
        .route("/api/competition/:id", get(competition))
        .route("/api/competitions", get(competitions))
        .route("/api/competition/fromhc/:compid", get(from_highcloud))
        .route("/api/competitions", post(create_competition))
        .route("/api/rankings", get(get_rankings))
        .route("/api/ranking/:date", get(get_ranking))
        .route("/api/rankings/:date", post(create_ranking));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
