use std::{
    str,
    process::Command,
};
use axum::{
    Json,
    debug_handler,
    extract::State,
};
use serde::{Deserialize, Serialize};
use crate::server::server::SharedState;


#[derive(Deserialize, Serialize)]
pub struct ProveRequest {
    pub input: String,
}

#[derive(Deserialize, Serialize)]
pub struct VerifyRequest {
    pub proof: String,
}

#[derive(Serialize)]
pub struct Response {
    pub status: String,
}

#[debug_handler]
pub async fn healthcheck() -> Json<Response> {
    Json(Response { status: "healthy".to_string() })
}

#[debug_handler]
pub async fn prove(State(state): State<SharedState>, Json(req): Json<ProveRequest>) -> Json<Response> {
    let _lock = state.0.write().await;
    let output = Command::new("ezkl")
        .arg("prove")
        .arg("--compiled-circuit")
        .arg(&req.input)
        .output()
        .expect("Failed to execute ezkl prove");

    let result = str::from_utf8(&output.stdout).unwrap_or("Error");
    Json(Response { status: result.to_string() })
}

#[debug_handler]
pub async fn verify(State(state): State<SharedState>, Json(req): Json<VerifyRequest>) -> Json<Response> {
    let _lock = state.0.write().await;
    let output = Command::new("ezkl")
        .arg("verify")
        .arg("--proof")
        .arg(&req.proof)
        .output()
        .expect("Failed to execute ezkl verify");

    let result = str::from_utf8(&output.stdout).unwrap_or("Error");
    Json(Response { status: result.to_string() })
}

