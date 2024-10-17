use std::{
    borrow::Cow,
    time::Duration,
};
use axum::{
    Router,
    BoxError,
    http::{StatusCode, Method},
    response::IntoResponse,
    error_handling::HandleErrorLayer,
    routing::{get, post},
};
use tower::ServiceBuilder;
use tower_http::{
    trace::{TraceLayer, DefaultOnRequest, DefaultOnResponse},
    cors::{Any, CorsLayer}
};


mod server;
mod handler;
mod error;
mod config;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    //TODO get from CLI
    let server = server::server::SharedState::new("./config.yaml".into()).await;

    let app = Router::new()
        .route("/api/v1/ping", get(||async{ "pong" }))
        .route("/api/v1/healthcheck", get(handler::handler::healthcheck))
        .route("/api/v1/prove", post(handler::handler::prove))
        .route("/api/v1/verify", post(handler::handler::verify))
        .layer(cors)
        .layer(
            ServiceBuilder::new()
            .layer(HandleErrorLayer::new(handle_error))
            .timeout(Duration::from_secs(600))
            .layer(TraceLayer::new_for_http()
            .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
            .on_response(DefaultOnResponse::new().level(tracing::Level::INFO))
            )
            )
        .with_state(server.clone());

    let addr = format!("{}:{}", server.0.read().await.config.server.host, server.0.read().await.config.server.port);
    tracing::info!("Server is running on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();


}

pub async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
            );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {error}")),
        )
}

