use crate::error::{ZKMLError, ZKMLResult};
use crate::server::server::SharedState;
use axum::{debug_handler, extract::State, Json};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use std::{process::Command, str};
use tempfile::NamedTempFile;

#[derive(Deserialize, Serialize)]
pub struct ProveRequest {
    pub req_id: String,
    pub input: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VerifyRequest {
    pub req_id: String,
    pub model: String,
    pub proof_path: String,
}

#[derive(Serialize)]
pub struct Response<T> {
    pub req_id: String,
    pub code: u16,
    pub result: T,
}

#[debug_handler]
pub async fn healthcheck() -> Json<Response<String>> {
    Json(Response {
        req_id: "".to_string(),
        code: 200,
        result: "healthy".to_string(),
    })
}

#[debug_handler]
pub async fn prove(
    State(state): State<SharedState>,
    Json(req): Json<ProveRequest>,
) -> Json<Response<String>> {
    let _lock = state.0.write().await;

    if req.input.is_empty() {
        return Json(Response {
            req_id: req.req_id.clone(),
            code: 500,
            result: "The input must not be empty".to_string(),
        });
    }

    let output = match Command::new("ezkl")
        .arg("prove")
        .arg("--compiled-circuit")
        .arg(&req.input)
        .output()
    {
        Ok(output) => output,
        Err(e) => {
            tracing::error!("Failed to execute ezkl prove: {}", e);
            return Json(Response {
                req_id: req.req_id.clone(),
                code: 500,
                result: "Failed to execute ezkl prove".to_string(),
            });
        }
    };

    let result = str::from_utf8(&output.stdout).unwrap_or("Error");
    Json(Response {
        req_id: req.req_id.clone(),
        code: 200,
        result: result.to_string(),
    })
}

#[debug_handler]
pub async fn verify(
    State(state): State<SharedState>,
    Json(req): Json<VerifyRequest>,
) -> Json<Response<String>> {
    tracing::info!("verify request: {:?}", &req);

    let _lock = state.0.write().await;

    let temp_proof_file = match fetch_to_temp_file(&req.proof_path).await {
        Ok(f) => f,
        Err(e) => {
            tracing::error!("read proof file error: {:?}", e);
            return Json(Response {
                req_id: req.req_id,
                code: e.error_code(),
                result: e.error_message(),
            });
        }
    };

    let model_root = Path::new(&_lock.config.public.models);
    let model_path = model_root.join(&req.model);
    if let Some(response) = check_path_exists(&model_path, &req.req_id, "Model directory") {
        return response;
    }

    let vk_path = model_path.join("vk.key");
    if let Some(response) = check_path_exists(&vk_path, &req.req_id, "VK") {
        return response;
    }

    let setting_path = model_path.join("settings.json");
    if let Some(response) = check_path_exists(&setting_path, &req.req_id, "settings") {
        return response;
    }

    let ezkl_bin = Path::new(&_lock.config.public.binfile);

    let output = match Command::new(ezkl_bin)
        .arg("verify")
        .arg("--proof-path")
        .arg(temp_proof_file.path())
        .arg("--vk-path")
        .arg(vk_path)
        .arg("--settings-path")
        .arg(setting_path)
        //.arg("--srs-path")
        //.arg(temp_kgz.into_temp_path())
        .output()
    {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("Failed to execute ezkl verify: {}", e);
            let err = ZKMLError::OtherError("Failed to execute ezkl verify".to_string());
            return Json(Response {
                req_id: req.req_id,
                code: err.error_code(),
                result: err.error_message(),
            });
        }
    };

    tracing::info!("{:?}", output);

    let result = str::from_utf8(&output.stdout).unwrap_or("Error");
    let res = if result.contains("verified: true") {
        true
    } else {
        false
    };

    Json(Response {
        req_id: req.req_id,
        code: 200,
        result: res.to_string(),
    })
}

async fn fetch_to_temp_file(url: &str) -> ZKMLResult<NamedTempFile> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| ZKMLError::OtherError(e.to_string()))?;

    if !response.status().is_success() {
        return Err(ZKMLError::OtherError("reponse is not success".to_string()));
    }

    let mut temp_file = NamedTempFile::new().map_err(|e| ZKMLError::OtherError(e.to_string()))?;
    let content = response
        .bytes()
        .await
        .map_err(|e| ZKMLError::OtherError(e.to_string()))?;
    temp_file
        .write_all(&content)
        .map_err(|e| ZKMLError::OtherError(e.to_string()))?;

    Ok(temp_file)
}

fn check_path_exists(
    path: &Path,
    req_id: &str,
    description: &str,
) -> Option<Json<Response<String>>> {
    if !path.exists() {
        tracing::error!("{} does not exist: {:?}", description, path);
        let err = ZKMLError::OtherError(format!("{} does not exist", description));
        return Some(Json(Response {
            req_id: req_id.to_string(),
            code: err.error_code(),
            result: err.error_message(),
        }));
    }
    None
}
