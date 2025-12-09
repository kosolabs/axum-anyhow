use anyhow::Result;
use axum::{extract::Path, routing::get, Json, Router};
use axum_anyhow::{set_error_enricher, ApiResult, ErrorInterceptorLayer, OptionExt, ResultExt};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Set up the error enricher to automatically add request metadata to all errors
    set_error_enricher(|builder, ctx| {
        *builder = builder.clone().meta(json!({
            "method": ctx.method.as_str(),
            "uri": ctx.uri.to_string(),
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }));
    });

    // Build the router with the error interceptor middleware
    let app = Router::new()
        .route("/users/{id}", get(get_user_handler))
        .layer(ErrorInterceptorLayer);

    println!("Server running on http://0.0.0.0:3000");
    println!("Try:");
    println!("  curl http://localhost:3000/users/1");
    println!("  curl http://localhost:3000/users/999  # Not found");
    println!("  curl http://localhost:3000/users/abc  # Bad request");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Serialize, Clone)]
struct User {
    id: u32,
    name: String,
}

/// Example handler - errors automatically include request metadata from the enricher
async fn get_user_handler(Path(id): Path<String>) -> ApiResult<Json<User>> {
    // Convert parsing errors to 400 Bad Request
    // The error will automatically include method, uri, and timestamp from the enricher
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
