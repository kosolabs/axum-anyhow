# axum-anyhow

[![Crates.io](https://img.shields.io/crates/v/axum-anyhow.svg)](https://crates.io/crates/axum-anyhow)
[![Documentation](https://docs.rs/axum-anyhow/badge.svg)](https://docs.rs/axum-anyhow)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A library for ergonomic error handling in Axum applications using anyhow.

This crate provides extension traits and utilities to easily convert `Result` and `Option` types into HTTP error responses with proper status codes, titles, and details.

> [!WARNING]
> This project is new and under active development. The API is still in flux and may have breaking changes between releases. While we follow [semantic versioning](https://semver.org/) to prevent changes from breaking your build, you may need to perform manual migration steps when upgrading to new versions. Please review the [CHANGELOG](CHANGELOG.md) when updating.

## Features

- Convert `anyhow::Result` to an `ApiError` with custom HTTP status codes.
- Convert `Option` to an `ApiError` when `None` is encountered.
- Returns JSON responses in [RFC 9457](https://www.rfc-editor.org/rfc/rfc9457.html) format.
- Optional environment variable to expose error details in development mode.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
anyhow = "1.0"
axum = "0.8"
axum-anyhow = "0"  # Use the latest 0.x version from crates.io
serde = "1.0"
tokio = { version = "1.48", features = ["full"] }
```

## Quick Start

```rust,no_run
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
```

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

async fn find_user(id: u32) -> ApiResult<String> {
    // Return 404 if user not found
    let user = database_lookup(id)
        .context_not_found("User Not Found", "No user with that ID exists")?;

    Ok(user)
}

fn database_lookup(id: u32) -> Option<String> {
    (id == 1).then(|| "Alice".to_string())
}
```

### Available Status Codes

The library provides helper methods for common HTTP status codes:

| Method                         | Status Code | Use Case                        |
| ------------------------------ | ----------- | ------------------------------- |
| `context_bad_request`          | 400         | Invalid client input            |
| `context_unauthorized`         | 401         | Authentication required         |
| `context_forbidden`            | 403         | Insufficient permissions        |
| `context_not_found`            | 404         | Resource doesn't exist          |
| `context_method_not_allowed`   | 405         | HTTP method not supported       |
| `context_conflict`             | 409         | Resource conflict               |
| `context_unprocessable_entity` | 422         | Validation errors               |
| `context_too_many_requests`    | 429         | Rate limit exceeded             |
| `context_internal`             | 500         | Server errors                   |
| `context_bad_gateway`          | 502         | Invalid upstream response       |
| `context_service_unavailable`  | 503         | Service temporarily unavailable |
| `context_gateway_timeout`      | 504         | Upstream timeout                |
| `context_status`               | Custom      | Any custom status code          |

### Creating Errors Directly

You can also create errors directly without Results or Options:

```rust
use axum_anyhow::{
    bad_request, unauthorized, forbidden, not_found, method_not_allowed,
    conflict, unprocessable_entity, too_many_requests, internal_error,
    bad_gateway, service_unavailable, gateway_timeout, ApiError
};
use axum::http::StatusCode;

// Using helper functions for common status codes
let error = bad_request("Invalid Input", "Name cannot be empty");
let error = unauthorized("Unauthorized", "Authentication token required");
let error = forbidden("Forbidden", "Insufficient permissions");
let error = not_found("Not Found", "Resource does not exist");
let error = method_not_allowed("Method Not Allowed", "POST not supported");
let error = conflict("Conflict", "Email already exists");
let error = unprocessable_entity("Validation Failed", "Password too short");
let error = too_many_requests("Rate Limited", "Try again in 60 seconds");
let error = internal_error("Internal Error", "Database connection failed");
let error = bad_gateway("Bad Gateway", "Upstream service error");
let error = service_unavailable("Service Unavailable", "Under maintenance");
let error = gateway_timeout("Gateway Timeout", "Upstream service timeout");

// Using the builder for custom status codes
let error = ApiError::builder()
    .status(StatusCode::IM_A_TEAPOT)
    .title("I'm a teapot")
    .detail("This server is a teapot, not a coffee maker")
    .build();
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

## Advanced Features

### Adding Metadata to Errors

You can include custom metadata in error responses using the `meta` field. This is useful for adding request IDs, trace information, timestamps, or other contextual data:

```rust
use axum::http::StatusCode;
use axum_anyhow::ApiError;
use serde_json::json;

let error = ApiError::builder()
    .status(StatusCode::NOT_FOUND)
    .title("User Not Found")
    .detail("No user with the given ID")
    .meta(json!({
        "request_id": "abc-123",
        "timestamp": "2024-01-01T12:00:00Z"
    }))
    .build();
```

This produces a JSON response like:

```json
{
  "status": 404,
  "title": "User Not Found",
  "detail": "No user with the given ID",
  "meta": {
    "request_id": "abc-123",
    "timestamp": "2024-01-01T12:00:00Z"
  }
}
```

### Request-Aware Error Metadata

For automatically enriching errors with request context (like request IDs, URIs, or headers), use the middleware and meta callback:

```rust,no_run
use axum::{Router, routing::get};
use axum_anyhow::{set_meta_callback, ErrorInterceptorLayer, ApiResult};
use serde_json::json;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // Set up a callback to enrich all errors with request metadata
    set_meta_callback(|meta, request| {
        *meta = Some(json!({
            "request_id": Uuid::new_v4().to_string(),
            "method": request.method().as_str(),
            "uri": request.uri().to_string(),
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }));
    });

    // Apply the middleware to your router
    let app: Router = Router::new()
        .route("/users/{id}", get(handler))
        .layer(ErrorInterceptorLayer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> ApiResult<String> {
    // Any error returned will automatically include the metadata
    // from the callback (request_id, method, uri, timestamp)
    Ok("Hello!".to_string())
}
```

You can also manually add metadata alongside automatic enrichment:

```rust
use axum_anyhow::{ApiError, ApiErrorBuilderExt};
use axum::extract::Request;
use serde_json::json;

fn create_error(request: &Request) -> ApiError {
    ApiError::builder()
        .status(axum::http::StatusCode::NOT_FOUND)
        .title("Resource Not Found")
        .detail("The requested resource does not exist")
        .meta(json!({"custom": "data"}))
        .with_request_meta(request)  // Also adds request context
        .build()
}
```

See the `examples/with-meta.rs` for a complete working example.

## Development Features

### Exposing Error Details

By default, when an `anyhow::Error` is automatically converted to an `ApiError` (via the `From` trait), the error detail is set to the generic message `"Something went wrong"`. This protects against accidentally leaking sensitive information in production.

However, during development, it can be helpful to see the actual error messages. You can enable this in two ways:

#### Programmatically (Recommended)

```rust
use axum_anyhow::set_expose_errors;

// Enable for development
set_expose_errors(true);

// Disable for production
set_expose_errors(false);
```

This is especially useful in tests or when you want fine-grained control:

```rust
use axum_anyhow::set_expose_errors;

#[cfg(debug_assertions)]
set_expose_errors(true);
```

#### Via Environment Variable

You can also set the `AXUM_ANYHOW_EXPOSE_ERRORS` environment variable:

```bash
AXUM_ANYHOW_EXPOSE_ERRORS=1 cargo run
# or
AXUM_ANYHOW_EXPOSE_ERRORS=true cargo run
```

With this enabled:

```rust
use anyhow::anyhow;
use axum_anyhow::ApiError;

// Without expose_errors: detail = "Something went wrong"
// With expose_errors: detail = "Database connection failed"
let error: ApiError = anyhow!("Database connection failed").into();
```

> [!WARNING]
> Error messages may contain sensitive information like file paths, database details, or internal system information that should not be exposed to end users in production.

## Motivation

Without `axum-anyhow`, the code in our quick start example would look like this:

```rust,no_run
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
```

Axum encourages you to create your own [error types and conversion logic](https://github.com/tokio-rs/axum/blob/main/examples/anyhow-error-response/src/main.rs) to reduce this boilerplate. `axum-anyhow` does this for you, providing extension traits and helper functions to convert standard Rust types (`Result` and `Option`) into properly formatted HTTP error responses.

`axum-anyhow` is designed for REST APIs and returns errors formatted according to [RFC 9457](https://www.rfc-editor.org/rfc/rfc9457.html). If you need more flexibility, please [file an issue](https://github.com/kosolabs/axum-anyhow/issues) or copy the code into your project and modify it as needed.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Repository

[https://github.com/kosolabs/axum-anyhow](https://github.com/kosolabs/axum-anyhow)
