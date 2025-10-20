# axum-anyhow

[![Crates.io](https://img.shields.io/crates/v/axum-anyhow.svg)](https://crates.io/crates/axum-anyhow)
[![Documentation](https://docs.rs/axum-anyhow/badge.svg)](https://docs.rs/axum-anyhow)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A library for ergonomic error handling in Axum applications using anyhow.

This crate provides extension traits and utilities to easily convert `Result` and `Option` types into HTTP error responses with proper status codes, titles, and details.

## Features

- Convert `anyhow::Result` to an `ApiError` with custom HTTP status codes.
- Convert `Option` to an `ApiError` when `None` is encountered.
- Returns JSON responses in [RFC 9457](https://www.rfc-editor.org/rfc/rfc9457.html) format.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
axum-anyhow = "0.2"
anyhow = "1.0"
axum = "0.8"
```

## Quick Start

```rust
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
```

## Motivation

Without `axum-anyhow`, error handling in Axum requires verbose boilerplate for each error case:

```rust
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
```

Axum encourages you to create your own [error types and conversion logic](https://github.com/tokio-rs/axum/blob/main/examples/anyhow-error-response/src/main.rs) to reduce this boilerplate. `axum-anyhow` basically does this for you. Additionally, it provides extension traits and helper functions to convert standard Rust types (`Result` and `Option`) into properly formatted HTTP error responses, eliminating the need to manually build error responses throughout your application.

`axum-anyhow` assumes that you are using Axum as a REST API server. The errors that `axum-anyhow` returns are REST errors formatted to conform to [RFC 9457](https://www.rfc-editor.org/rfc/rfc9457.html). If you want to use `axum-anyhow` but it isn't sufficiently flexible for your use case, please [file an issue](https://github.com/kosolabs/axum-anyhow/issues), or lift our code directly into your project, and modify it to be as flexible as you need it to be.

## Usage Examples

### Working with Results

Use the `ResultExt` trait to convert any `anyhow::Result` into an HTTP error response:

```rust
use axum_anyhow::{ApiResult, ResultExt};
use anyhow::Result;

async fn validate_email(email: String) -> ApiResult<String> {
    // Validate and return 400 if invalid
    check_email_format(&email)
        .context_bad_request("Invalid Email", "Email must contain @")?;

    Ok(email)
}

fn check_email_format(email: &str) -> Result<()> {
    if email.contains('@') {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Invalid format"))
    }
}
```

### Working with Options

Use the `OptionExt` trait to convert `Option` into an HTTP error response:

```rust
use axum_anyhow::{ApiResult, OptionExt};

async fn find_user(id: u32) -> ApiResult<User> {
    // Return 404 if user not found
    let user = database_lookup(id)
        .context_not_found("User Not Found", "No user with that ID exists")?;

    Ok(user)
}
```

### Available Status Codes

The library provides helper methods for common HTTP status codes:

| Method                    | Status Code | Use Case                 |
| ------------------------- | ----------- | ------------------------ |
| `context_bad_request`     | 400         | Invalid client input     |
| `context_unauthenticated` | 401         | Authentication required  |
| `context_unauthorized`    | 403         | Insufficient permissions |
| `context_not_found`       | 404         | Resource doesn't exist   |
| `context_internal`        | 500         | Server errors            |
| `context_status`          | Custom      | Any custom status code   |

### Creating Errors Directly

You can also create errors directly without Results or Options:

```rust
use axum_anyhow::{bad_request, not_found, unauthorized, ApiError};
use axum::http::StatusCode;

// Using helper functions
let error = bad_request("Invalid Input", "Name cannot be empty");

// Using the generic constructor
let error = api_error(
    StatusCode::CONFLICT,
    "Conflict",
    "A user with this email already exists"
);
```

### Error Response Format

All errors are serialized as JSON with the following structure:

```json
{
  "status": 404,
  "title": "Not Found",
  "detail": "The requested user does not exist"
}
```

## Complete Example

Here's a complete example showing different error scenarios:

```rust
use anyhow::{anyhow, Result};
use axum::{routing::get, Json, Router};
use axum_anyhow::{ApiResult, OptionExt, ResultExt};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ok", get(handler_ok))
        .route("/result/400", get(handler_bad_request_result))
        .route("/option/404", get(handler_not_found_option))
        .route("/result/500", get(handler_internal_error));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Serialize)]
struct Response {
    message: String,
}

async fn handler_ok() -> ApiResult<Json<Response>> {
    Ok(Json(Response {
        message: "Success!".to_string(),
    }))
}

async fn handler_bad_request_result() -> ApiResult<Json<Response>> {
    validate_input()
        .context_bad_request("Bad Request", "Input validation failed")?;
    Ok(Json(Response {
        message: "Valid".to_string(),
    }))
}

async fn handler_not_found_option() -> ApiResult<Json<Response>> {
    find_resource()
        .context_not_found("Not Found", "The requested resource does not exist")?;
    Ok(Json(Response {
        message: "Found".to_string(),
    }))
}

async fn handler_internal_error() -> ApiResult<Json<Response>> {
    // Errors without explicit context become 500 errors
    database_operation()?;
    Ok(Json(Response {
        message: "Success".to_string(),
    }))
}

fn validate_input() -> Result<()> {
    Err(anyhow!("Validation failed"))
}

fn find_resource() -> Option<String> {
    None
}

fn database_operation() -> Result<()> {
    Err(anyhow!("Database connection failed"))
}
```

## Why Use This Library?

### Before

```rust
async fn get_user(id: u32) -> Result<Json<User>, (StatusCode, Json<ErrorResponse>)> {
    let user = match database.find_user(id) {
        Some(u) => u,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    status: 404,
                    title: "Not Found".to_string(),
                    detail: "User not found".to_string(),
                }),
            ));
        }
    };
    Ok(Json(user))
}
```

### After

```rust
async fn get_user(id: u32) -> ApiResult<Json<User>> {
    let user = database.find_user(id)
        .context_not_found("Not Found", "User not found")?;
    Ok(Json(user))
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Repository

[https://github.com/kosolabs/axum-anyhow](https://github.com/kosolabs/axum-anyhow)
