use anyhow::{anyhow, Result};
use axum::{routing::get, Json, Router};
use axum_anyhow::{ApiResult, OptionExt, ResultExt};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ok", get(handler_ok))
        .route("/result/400", get(handler_bad_request_result))
        .route("/option/400", get(handler_bad_request_option))
        .route("/result/500", get(handler_internal_server_error_result));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn error_result() -> Result<()> {
    Err(anyhow!("An error occurred"))
}

fn error_option() -> Option<()> {
    None
}

#[derive(serde::Serialize)]
struct Response {}

async fn handler_ok() -> ApiResult<Json<Response>> {
    Ok(Json(Response {}))
}

async fn handler_bad_request_result() -> ApiResult<Json<Response>> {
    error_result().context_bad_request("Bad Request", "The result was Err")?;
    Ok(Json(Response {}))
}

async fn handler_bad_request_option() -> ApiResult<Json<Response>> {
    error_option().ok_or_bad_request("Bad Request", "The option was None")?;
    Ok(Json(Response {}))
}

async fn handler_internal_server_error_result() -> ApiResult<Json<Response>> {
    error_result()?;
    Ok(Json(Response {}))
}
