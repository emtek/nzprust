use axum::{
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{get, get_service},
    Extension, Json, Router,
};
use competitions::{competition_routes, restricted_competition_routes};
use data::data_access::*;
use frontend::prs_data_types::UserInfo;
use google_auth::google_auth;
use google_signin::CachedCerts;
use integrations::{from_fai, from_highcloud};
use opentelemetry::{
    global::{self},
    sdk::{propagation::TraceContextPropagator, Resource},
    KeyValue,
};
use opentelemetry::{
    runtime::Tokio,
    sdk::trace::{self},
};
use opentelemetry_otlp::WithExportConfig;
use pilots::pilot_routes;
use rankings::ranking_routes;
use std::{collections::HashMap, env, net::SocketAddr, path::PathBuf};
use tokio::signal;
use tower_http::{catch_panic::CatchPanicLayer, services::ServeFile, trace::TraceLayer};
use tracing::instrument::WithSubscriber;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{filter, Layer};
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
    let admin_users: Vec<String> = data.admin_users.iter().map(|f| f.clone()).collect();
    let google_certs = CachedCerts::new();
    Router::new()
        .fallback(static_files_service)
        .route("/api/profile", get(get_profile))
        .merge(restricted_competition_routes())
        .route_layer(middleware::from_fn_with_state(
            (admin_users.clone(), google_certs.clone()),
            google_auth,
        ))
        .route("/api/competition/fromhc/:compid", get(from_highcloud))
        .route("/api/competition/fromfai/:compid", get(from_fai))
        .merge(competition_routes())
        .merge(pilot_routes())
        .merge(ranking_routes())
        .with_state(data)
        .layer(CatchPanicLayer::new())
        .layer(TraceLayer::new_for_http())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Shutdown signal received, starting graceful shutdown");
}

#[tokio::main]
async fn main() {
    // Setting a trace context propagation data.
    global::set_text_map_propagator(TraceContextPropagator::new());
    // initialize tracing output to stdout
    let filter = filter::Targets::new()
        .with_target("tower_http::trace::on_response", tracing::Level::TRACE)
        .with_target("tower_http::trace::on_request", tracing::Level::TRACE)
        .with_target("tower_http::trace::on_failure", tracing::Level::ERROR)
        .with_target("hyper", tracing::Level::ERROR)
        .with_default(tracing::Level::DEBUG);
    let layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(filter.clone());

    if let Ok(api_key) = env::var("HONEYCOMB_API_KEY") {
        let exporter = opentelemetry_otlp::new_exporter()
            .http()
            .with_endpoint("https://api.honeycomb.io/v1/traces")
            .with_http_client(reqwest::Client::default())
            .with_headers(HashMap::from([("x-honeycomb-team".into(), api_key)]));
        let otlp_tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(exporter)
            .with_trace_config(
                trace::config().with_resource(Resource::new(vec![KeyValue::new(
                    opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                    "NZPRS".to_string(),
                )])),
            )
            .install_batch(opentelemetry::runtime::Tokio)
            .expect("Error - Failed to create tracer.");

        tracing_subscriber::registry().with(
            tracing_opentelemetry::layer()
                .with_tracer(otlp_tracer)
                .with_filter(filter),
        );
    } else {
        // just print to console
        tracing_subscriber::registry().with(layer).init();
    }

    let router = setup_server();
    // run our app with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::info!("nzprs backend listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .with_current_subscriber()
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
