use anyhow::Result;
use axum::{routing::get, Json, Router};
use axum_anyhow::{ApiResult, OptionExt, ResultExt};

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

async fn get_user_handler(id: String) -> ApiResult<Json<User>> {
    // Convert Result errors to 400 Bad Request
    let id = parse_id(&id).context_bad_request("Invalid User ID", "User ID must be a u32")?;

    // Convert Option::None to 404 Not Found
    let user = fetch_user(id).context_not_found("User Not Found", "No user with that ID")?;

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
