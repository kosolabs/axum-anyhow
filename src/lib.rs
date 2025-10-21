//! # axum-anyhow
//!
//! A library for ergonomic error handling in Axum applications using anyhow.
//!
//! This crate provides extension traits and utilities to easily convert `Result` and
//! `Option` types into HTTP error responses with proper status codes, titles, and
//! details.
//!
//! ## Example
//!
//! ```rust
//! use axum::{routing::get, Json, Router};
//! use axum_anyhow::{ApiResult, ResultExt, OptionExt};
//! use anyhow::{anyhow, Result};
//!
//! #[derive(serde::Serialize)]
//! struct User {
//!     id: u32,
//!     name: String,
//! }
//!
//! async fn handler(id: String) -> ApiResult<Json<User>> {
//!     // Convert Result errors to 400 Bad Request
//!     let id = parse_id(&id).context_bad_request("Invalid User ID", "User ID must be a u32")?;
//!
//!     // Convert Option::None to 404 Not Found
//!     let user = fetch_user(id).context_not_found("User Not Found", "No user with that ID")?;
//!
//!     Ok(Json(user))
//! }
//!
//! fn fetch_user(id: u32) -> Option<User> {
//!     (id == 1).then(|| User {
//!         id,
//!         name: "Alice".to_string(),
//!     })
//! }
//!
//! fn parse_id(id: &str) -> Result<u32> {
//!     Ok(id.parse::<u32>()?)
//! }
//! ```

mod error;
mod extensions;

pub use error::{ApiError, ApiErrorBuilder};
pub use extensions::{IntoApiError, OptionExt, ResultExt};

use anyhow::Result;
use axum::http::StatusCode;

/// A type alias for `Result<T, ApiError>`.
///
/// Use this as the return type for Axum handlers to automatically convert errors
/// into HTTP responses.
///
/// # Example
///
/// ```rust
/// use axum::Json;
/// use axum_anyhow::ApiResult;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Response {
///     message: String,
/// }
///
/// async fn handler() -> ApiResult<Json<Response>> {
///     Ok(Json(Response {
///         message: "Success".to_string(),
///     }))
/// }
/// ```
pub type ApiResult<T> = Result<T, ApiError>;

/// Creates a 400 Bad Request error.
///
/// # Arguments
///
/// * `title` - A short, human-readable summary of the error
/// * `detail` - A detailed explanation of the error
///
/// # Example
///
/// ```rust
/// use axum_anyhow::bad_request;
///
/// let error = bad_request("Invalid Input", "Email format is invalid");
/// ```
pub fn bad_request(title: &str, detail: &str) -> ApiError {
    ApiError::builder()
        .status(StatusCode::BAD_REQUEST)
        .title(title)
        .detail(detail)
        .build()
}

/// Creates a 401 Unauthorized error (for authentication failures).
///
/// # Arguments
///
/// * `title` - A short, human-readable summary of the error
/// * `detail` - A detailed explanation of the error
///
/// # Example
///
/// ```rust
/// use axum_anyhow::unauthenticated;
///
/// let error = unauthenticated("Unauthenticated", "No valid authentication token provided");
/// ```
pub fn unauthenticated(title: &str, detail: &str) -> ApiError {
    ApiError::builder()
        .status(StatusCode::UNAUTHORIZED)
        .title(title)
        .detail(detail)
        .build()
}

/// Creates a 403 Forbidden error (for authorization failures).
///
/// # Arguments
///
/// * `title` - A short, human-readable summary of the error
/// * `detail` - A detailed explanation of the error
///
/// # Example
///
/// ```rust
/// use axum_anyhow::unauthorized;
///
/// let error = unauthorized("Forbidden", "You do not have permission to access this resource");
/// ```
pub fn unauthorized(title: &str, detail: &str) -> ApiError {
    ApiError::builder()
        .status(StatusCode::FORBIDDEN)
        .title(title)
        .detail(detail)
        .build()
}

/// Creates a 404 Not Found error.
///
/// # Arguments
///
/// * `title` - A short, human-readable summary of the error
/// * `detail` - A detailed explanation of the error
///
/// # Example
///
/// ```rust
/// use axum_anyhow::not_found;
///
/// let error = not_found("Not Found", "The requested user does not exist");
/// ```
pub fn not_found(title: &str, detail: &str) -> ApiError {
    ApiError::builder()
        .status(StatusCode::NOT_FOUND)
        .title(title)
        .detail(detail)
        .build()
}

/// Creates a 500 Internal Server Error.
///
/// # Arguments
///
/// * `title` - A short, human-readable summary of the error
/// * `detail` - A detailed explanation of the error
///
/// # Example
///
/// ```rust
/// use axum_anyhow::internal_error;
///
/// let error = internal_error("Internal Error", "Database connection failed");
/// ```
pub fn internal_error(title: &str, detail: &str) -> ApiError {
    ApiError::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .title(title)
        .detail(detail)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bad_request_helper() {
        let err = bad_request("Bad Request", "Invalid input");
        assert_eq!(err.status, StatusCode::BAD_REQUEST);
        assert_eq!(err.title, "Bad Request");
        assert_eq!(err.detail, "Invalid input");
    }

    #[test]
    fn test_unauthenticated_helper() {
        let err = unauthenticated("Unauthenticated", "No token provided");
        assert_eq!(err.status, StatusCode::UNAUTHORIZED);
        assert_eq!(err.title, "Unauthenticated");
        assert_eq!(err.detail, "No token provided");
    }

    #[test]
    fn test_unauthorized_helper() {
        let err = unauthorized("Unauthorized", "Insufficient permissions");
        assert_eq!(err.status, StatusCode::FORBIDDEN);
        assert_eq!(err.title, "Unauthorized");
        assert_eq!(err.detail, "Insufficient permissions");
    }

    #[test]
    fn test_not_found_helper() {
        let err = not_found("Not Found", "Resource does not exist");
        assert_eq!(err.status, StatusCode::NOT_FOUND);
        assert_eq!(err.title, "Not Found");
        assert_eq!(err.detail, "Resource does not exist");
    }

    #[test]
    fn test_internal_error_helper() {
        let err = internal_error("Internal Error", "Database connection failed");
        assert_eq!(err.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(err.title, "Internal Error");
        assert_eq!(err.detail, "Database connection failed");
    }
}
