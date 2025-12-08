use anyhow::Result;
use axum::{extract::Path, routing::get, Json, Router};
use axum_anyhow::{set_meta_callback, ApiResult, ErrorInterceptorLayer, OptionExt, ResultExt};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // Set up the meta callback to enrich errors with request context
    set_meta_callback(|meta, request| {
        // Generate a unique request ID for tracing
        let request_id = Uuid::new_v4().to_string();

        *meta = Some(json!({
            "request_id": request_id,
            "method": request.method().as_str(),
            "uri": request.uri().to_string(),
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }));
    });

    // Build the router with the error interceptor middleware
    let app = Router::new()
        .route("/users/:id", get(get_user_handler))
        .route("/manual-meta/:id", get(manual_meta_handler))
        .layer(ErrorInterceptorLayer);

    println!("Server running on http://0.0.0.0:3000");
    println!("Try:");
    println!("  curl http://localhost:3000/users/1");
    println!("  curl http://localhost:3000/users/999  # Not found with meta");
    println!("  curl http://localhost:3000/users/abc  # Bad request with meta");
    println!("  curl http://localhost:3000/manual-meta/123  # Manual meta example");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Serialize, Clone)]
struct User {
    id: u32,
    name: String,
}

/// Example handler that uses automatic meta enrichment from the middleware
async fn get_user_handler(Path(id): Path<String>) -> ApiResult<Json<User>> {
    // Convert parsing errors to 400 Bad Request
    // The error will automatically include meta from the callback
    let id = parse_id(&id).context_bad_request("Invalid User ID", "User ID must be a u32")?;

    // Convert unexpected errors to 500 Internal Server Error
    let db = Database::connect()?;

    // Convert Option::None to 404 Not Found
    // The error will automatically include request metadata
    let user = db
        .get_user(&id)
        .context_not_found("User Not Found", "No user with that ID")?;

    Ok(Json(user))
}

/// Example handler that manually sets custom meta alongside automatic enrichment
async fn manual_meta_handler(Path(id): Path<String>) -> ApiResult<Json<User>> {
    let id = parse_id(&id).context_bad_request("Invalid User ID", "User ID must be a u32")?;
    let db = Database::connect()?;

    // Get user or return error with custom meta
    let user = match db.get_user(&id) {
        Some(user) => user,
        None => {
            // Manually create error with custom metadata
            // Note: The middleware meta callback will still be applied
            return Err(axum_anyhow::ApiError::builder()
                .status(axum::http::StatusCode::NOT_FOUND)
                .title("User Not Found")
                .detail(format!("No user found with ID {}", id))
                .meta(json!({
                    "attempted_id": id,
                    "available_ids": db.list_user_ids(),
                    "suggestion": "Try using a valid user ID from the available list"
                }))
                .build());
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

    fn list_user_ids(&self) -> Vec<u32> {
        self.users.keys().copied().collect()
    }
}

fn parse_id(id: &str) -> Result<u32> {
    Ok(id.parse::<u32>()?)
}
