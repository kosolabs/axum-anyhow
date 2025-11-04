use anyhow::Result;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{routing::get, Json, Router};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/users/{id}", get(get_user_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Serialize, Clone)]
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

async fn get_user_handler(
    Path(id): Path<String>,
) -> Result<Json<User>, (StatusCode, Json<ErrorResponse>)> {
    // Convert parsing errors to 400 Bad Request
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

    // Convert unexpected errors to 500 Internal Server Error
    let db = match Database::connect() {
        Ok(db) => db,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    status: 500,
                    title: "Internal Error".to_string(),
                    detail: "Something went wrong".to_string(),
                }),
            ));
        }
    };

    // Convert Option::None to 404 Not Found
    let user = match db.get_user(&id) {
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

// Mock database
struct Database {
    users: HashMap<u32, &'static str>,
}

impl Database {
    fn connect() -> Result<Self> {
        Ok(Database {
            users: HashMap::from([(1, "Alice"), (2, "Bob"), (3, "Eve")]),
        })
    }

    fn get_user(&self, id: &u32) -> Option<User> {
        self.users.get(id).map(|name| User {
            id: *id,
            name: name.to_string(),
        })
    }
}

fn parse_id(id: &str) -> Result<u32> {
    Ok(id.parse::<u32>()?)
}
