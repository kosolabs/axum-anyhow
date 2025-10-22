use anyhow::Result;
use axum::{extract::Path, routing::get, Json, Router};
use axum_anyhow::{ApiResult, OptionExt, ResultExt};
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

async fn get_user_handler(Path(id): Path<String>) -> ApiResult<Json<User>> {
    // Convert parsing errors to 400 Bad Request
    let id = parse_id(&id).context_bad_request("Invalid User ID", "User ID must be a u32")?;

    // Convert unexpected errors to 500 Internal Server Error
    let db = Database::connect()?;

    // Convert Option::None to 404 Not Found
    let user = db
        .get_user(&id)
        .context_not_found("User Not Found", "No user with that ID")?;

    Ok(Json(user))
}

// Mock database
struct Database {
    users: HashMap<u32, User>,
}

impl Database {
    fn connect() -> Result<Self> {
        // Simulate database connection with sample data
        #[rustfmt::skip]
        let users = HashMap::from([
            (1, User { id: 1, name: "Alice".to_string() }),
            (2, User { id: 2, name: "Bob".to_string() }),
        ]);
        Ok(Database { users })
    }

    fn get_user(&self, id: &u32) -> Option<User> {
        self.users.get(id).cloned()
    }
}

fn parse_id(id: &str) -> Result<u32> {
    Ok(id.parse::<u32>()?)
}
