use anyhow::Result;
use axum::http::StatusCode;
use axum::{routing::get, Json, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/users/:id", get(get_user_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Serialize)]
struct User {
    id: u32,
    name: String,
}

#[derive(serde::Serialize)]
struct ErrorResponse {
    status: u16,
    title: String,
    detail: String,
}

async fn get_user_handler(id: String) -> Result<Json<User>, (StatusCode, Json<ErrorResponse>)> {
    // Convert Result errors to 400 Bad Request
    let id = match parse_id(&id) {
        Ok(id) => id,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    status: 400,
                    title: "Invalid User ID".to_string(),
                    detail: "User ID must be a u32".to_string(),
                }),
            ));
        }
    };

    // Convert Option::None to 404 Not Found
    let user = match fetch_user(id) {
        Some(u) => u,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    status: 404,
                    title: "User Not Found".to_string(),
                    detail: "No user with that ID".to_string(),
                }),
            ));
        }
    };

    Ok(Json(user))
}

fn fetch_user(id: u32) -> Option<User> {
    (id == 1).then(|| User {
        id,
        name: "Alice".to_string(),
    })
}

fn parse_id(id: &str) -> Result<u32> {
    Ok(id.parse::<u32>()?)
}
