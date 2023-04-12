use axum::{
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{get, get_service},
    Extension, Json, Router,
};

use competitions::competition_routes;
use data::data_access::*;
use frontend::prs_data_types::UserInfo;
use google_auth::google_auth;
use integrations::{from_fai, from_highcloud};
use pilots::pilot_routes;
use rankings::ranking_routes;
use std::{net::SocketAddr, path::PathBuf};
use tower_http::services::ServeFile;
mod competitions;
mod data;
mod google_auth;
mod integrations;
mod pilots;
mod rankings;
mod scoring;

async fn get_profile(Extension(profile): Extension<UserInfo>) -> Response {
    (StatusCode::OK, Json(profile)).into_response()
}

fn setup_server() -> Router {
    let assets_dir = PathBuf::from("./dist");
    let static_files_service = get_service(
        tower_http::services::ServeDir::new(assets_dir)
            .append_index_html_on_directories(true)
            .fallback(ServeFile::new("./dist/index.html")),
    );
    let data = load_data().unwrap();
    Router::new()
        .fallback(static_files_service)
        .route("/api/profile", get(get_profile))
        .route_layer(middleware::from_fn_with_state(data.clone(), google_auth))
        .route("/api/competition/fromhc/:compid", get(from_highcloud))
        .route("/api/competition/fromfai/:compid", get(from_fai))
        .merge(competition_routes(data.clone()))
        .merge(pilot_routes())
        .merge(ranking_routes())
        .with_state(data)
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let router = setup_server();

    // run our app with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_should_be_valid() {
        let _ = setup_server();
    }
}
