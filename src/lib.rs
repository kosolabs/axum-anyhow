//! # axum-anyhow
//!
//! A library for ergonomic error handling in Axum applications using anyhow.
//!
//! This crate provides extension traits and utilities to easily convert `Result` and
//! `Option` types into HTTP error responses with proper status codes, titles, and
//! details.
//!
//!
//! ## Features
//!
//! - Convert `anyhow::Result` to API errors with custom HTTP status codes
//! - Convert `Option` to API errors when `None` is encountered
//! - Helper functions for common HTTP error codes (400, 401, 403, 404, 500)
//! - Automatic JSON serialization of error responses
//! - Seamless integration with Axum's `IntoResponse` trait
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
//! async fn get_user(id: u32) -> ApiResult<Json<User>> {
//!     // Convert Result errors to 400 Bad Request
//!     let user_data = fetch_user(id)
//!         .context_bad_request("Invalid User", "User data is invalid")?;
//!     
//!     // Convert Option::None to 404 Not Found
//!     let user = parse_user(user_data)
//!         .ok_or_not_found("User Not Found", "No user with that ID")?;
//!     
//!     Ok(Json(user))
//! }
//!
//! fn fetch_user(id: u32) -> Result<String> {
//!     // ... implementation
//! #   Ok("user data".to_string())
//! }
//!
//! fn parse_user(data: String) -> Option<User> {
//!     // ... implementation
//! #   Some(User { id: 1, name: "Alice".to_string() })
//! }
//! ```

use anyhow::Result;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

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

/// Extension trait for `anyhow::Result` to convert errors into `ApiError` with HTTP
/// status codes.
///
/// This trait provides methods to convert `Result<T, E>` where `E: Into<anyhow::Error>`
/// into `ApiResult<T>`, attaching HTTP status codes and error details.
///
/// # Example
///
/// ```rust
/// use anyhow::{anyhow, Result};
/// use axum_anyhow::{ApiResult, ResultExt};
/// use axum::http::StatusCode;
///
/// fn validate_email(email: &str) -> Result<String> {
///     if email.contains('@') {
///         Ok(email.to_string())
///     } else {
///         Err(anyhow!("Invalid email format"))
///     }
/// }
///
/// async fn handler(email: String) -> ApiResult<String> {
///     let validated = validate_email(&email)
///         .context_bad_request("Invalid Email", "Email must contain @")?;
///     Ok(validated)
/// }
///
/// # tokio_test::block_on(async {
/// let api_result = handler("not-an-email".to_string()).await;
/// assert!(api_result.is_err());
/// let err = api_result.unwrap_err();
/// assert_eq!(err.status, StatusCode::BAD_REQUEST);
/// assert_eq!(err.title, "Invalid Email");
/// assert_eq!(err.detail, "Email must contain @");
/// # })
/// ```
pub trait ResultExt<T> {
    /// Converts an error to an `ApiError` with a custom status code.
    ///
    /// # Arguments
    ///
    /// * `status` - The HTTP status code to use
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_status(self, status: StatusCode, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts an error to a 400 Bad Request error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_bad_request(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts an error to a 401 Unauthorized error (for authentication failures).
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_unauthenticated(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts an error to a 403 Forbidden error (for authorization failures).
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_unauthorized(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts an error to a 404 Not Found error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_not_found(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts an error to a 500 Internal Server Error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_internal(self, title: &str, detail: &str) -> ApiResult<T>;
}

impl<T> ResultExt<T> for Result<T> {
    fn context_status(self, status: StatusCode, title: &str, detail: &str) -> ApiResult<T> {
        self.map_err(|err| err.context_status(status, title, detail))
    }

    fn context_bad_request(self, title: &str, detail: &str) -> ApiResult<T> {
        self.map_err(|err| err.context_bad_request(title, detail))
    }

    fn context_unauthenticated(self, title: &str, detail: &str) -> ApiResult<T> {
        self.map_err(|err| err.context_unauthenticated(title, detail))
    }

    fn context_unauthorized(self, title: &str, detail: &str) -> ApiResult<T> {
        self.map_err(|err| err.context_unauthorized(title, detail))
    }

    fn context_not_found(self, title: &str, detail: &str) -> ApiResult<T> {
        self.map_err(|err| err.context_not_found(title, detail))
    }

    fn context_internal(self, title: &str, detail: &str) -> ApiResult<T> {
        self.map_err(|err| err.context_internal(title, detail))
    }
}

/// Extension trait for `Option<T>` to convert `None` into `ApiError` with HTTP status codes.
///
/// This trait provides methods to convert `Option<T>` into `ApiResult<T>`, converting
/// `None` values into errors with appropriate HTTP status codes and error details.
///
/// # Example
///
/// ```rust
/// use axum_anyhow::{ApiResult, OptionExt};
/// use axum::http::StatusCode;
///
///
/// fn find_user(id: u32) -> Option<String> {
///     if (id == 0) {
///         Some("Alice".to_string())
///     } else {
///         None
///     }
/// }
///
/// async fn handler(id: u32) -> ApiResult<String> {
///     let user = find_user(id)
///         .ok_or_not_found("User Not Found", "No user with that ID exists")?;
///     Ok(user)
/// }
///
/// # tokio_test::block_on(async {
/// let api_result = handler(1).await;
/// assert!(api_result.is_err());
/// let err = api_result.unwrap_err();
/// assert_eq!(err.status, StatusCode::NOT_FOUND);
/// assert_eq!(err.title, "User Not Found");
/// assert_eq!(err.detail, "No user with that ID exists");
/// # })
/// ```
pub trait OptionExt<T> {
    /// Converts `None` to an `ApiError` with a custom status code.
    ///
    /// # Arguments
    ///
    /// * `status` - The HTTP status code to use
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn ok_or_status(self, status: StatusCode, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 400 Bad Request error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn ok_or_bad_request(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 401 Unauthorized error (for authentication failures).
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn ok_or_unauthenticated(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 403 Forbidden error (for authorization failures).
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn ok_or_unauthorized(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 404 Not Found error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn ok_or_not_found(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 500 Internal Server Error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn ok_or_internal(self, title: &str, detail: &str) -> ApiResult<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_status(self, status: StatusCode, title: &str, detail: &str) -> ApiResult<T> {
        self.ok_or_else(|| api_error(status, title, detail))
    }

    fn ok_or_bad_request(self, title: &str, detail: &str) -> ApiResult<T> {
        self.ok_or_status(StatusCode::BAD_REQUEST, title, detail)
    }

    fn ok_or_unauthenticated(self, title: &str, detail: &str) -> ApiResult<T> {
        self.ok_or_status(StatusCode::UNAUTHORIZED, title, detail)
    }

    fn ok_or_unauthorized(self, title: &str, detail: &str) -> ApiResult<T> {
        self.ok_or_status(StatusCode::FORBIDDEN, title, detail)
    }

    fn ok_or_not_found(self, title: &str, detail: &str) -> ApiResult<T> {
        self.ok_or_status(StatusCode::NOT_FOUND, title, detail)
    }

    fn ok_or_internal(self, title: &str, detail: &str) -> ApiResult<T> {
        self.ok_or_status(StatusCode::INTERNAL_SERVER_ERROR, title, detail)
    }
}

/// Extension trait for converting any error type into `ApiError` with HTTP status codes.
///
/// This trait is implemented for all types that can be converted into `anyhow::Error`.
/// It provides methods to directly convert errors into `ApiError` instances with
/// specific HTTP status codes.
///
/// # Example
///
/// ```rust
/// use anyhow::anyhow;
/// use axum_anyhow::{ApiError, IntoApiError};
///
/// let error = anyhow!("Something went wrong");
/// let api_error: ApiError = error.context_internal("Internal Error", "Database failed");
/// ```
pub trait IntoApiError<E> {
    /// Converts an error to an `ApiError` with a custom status code.
    ///
    /// # Arguments
    ///
    /// * `status` - The HTTP status code to use
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_status(self, status: StatusCode, title: &str, detail: &str) -> ApiError;

    /// Converts an error to a 400 Bad Request error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_bad_request(self, title: &str, detail: &str) -> ApiError;

    /// Converts an error to a 401 Unauthorized error (for authentication failures).
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_unauthenticated(self, title: &str, detail: &str) -> ApiError;

    /// Converts an error to a 403 Forbidden error (for authorization failures).
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_unauthorized(self, title: &str, detail: &str) -> ApiError;

    /// Converts an error to a 404 Not Found error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_not_found(self, title: &str, detail: &str) -> ApiError;

    /// Converts an error to a 500 Internal Server Error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_internal(self, title: &str, detail: &str) -> ApiError;
}

impl<E> IntoApiError<E> for E
where
    E: Into<anyhow::Error>,
{
    fn context_status(self, status: StatusCode, title: &str, detail: &str) -> ApiError {
        api_error(status, title, detail)
    }

    fn context_bad_request(self, title: &str, detail: &str) -> ApiError {
        self.context_status(StatusCode::BAD_REQUEST, title, detail)
    }

    fn context_unauthenticated(self, title: &str, detail: &str) -> ApiError {
        self.context_status(StatusCode::UNAUTHORIZED, title, detail)
    }

    fn context_unauthorized(self, title: &str, detail: &str) -> ApiError {
        self.context_status(StatusCode::FORBIDDEN, title, detail)
    }

    fn context_not_found(self, title: &str, detail: &str) -> ApiError {
        self.context_status(StatusCode::NOT_FOUND, title, detail)
    }

    fn context_internal(self, title: &str, detail: &str) -> ApiError {
        self.context_status(StatusCode::INTERNAL_SERVER_ERROR, title, detail)
    }
}

/// Creates an `ApiError` with the specified status code, title, and detail.
///
/// # Arguments
///
/// * `status` - The HTTP status code
/// * `title` - A short, human-readable summary of the error
/// * `detail` - A detailed explanation of the error
///
/// # Example
///
/// ```rust
/// use axum::http::StatusCode;
/// use axum_anyhow::api_error;
///
/// let error = api_error(
///     StatusCode::CONFLICT,
///     "Conflict",
///     "A user with this email already exists"
/// );
/// ```
pub fn api_error(status: StatusCode, title: &str, detail: &str) -> ApiError {
    ApiError {
        status,
        title: title.to_string(),
        detail: detail.to_string(),
    }
}

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
    api_error(StatusCode::BAD_REQUEST, title, detail)
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
    api_error(StatusCode::UNAUTHORIZED, title, detail)
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
    api_error(StatusCode::FORBIDDEN, title, detail)
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
    api_error(StatusCode::NOT_FOUND, title, detail)
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
    api_error(StatusCode::INTERNAL_SERVER_ERROR, title, detail)
}

/// Converts from `anyhow::Error` to `ApiError`.
///
/// By default, all errors are converted to 500 Internal Server Error responses.
/// Use the extension traits to specify different status codes.
impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        err.context_internal("Internal Error", "Something went wrong")
    }
}

/// An API error that can be converted into an HTTP response.
///
/// This struct contains the HTTP status code, a title, and a detailed description
/// of the error. When converted into a response, it produces a JSON body with
/// these fields.
///
/// # JSON Response Format
///
/// ```json
/// {
///   "status": 404,
///   "title": "Not Found",
///   "detail": "The requested resource does not exist"
/// }
/// ```
///
/// # Example
///
/// ```rust
/// use axum::http::StatusCode;
/// use axum_anyhow::ApiError;
///
/// let error = ApiError {
///     status: StatusCode::NOT_FOUND,
///     title: "Not Found".to_string(),
///     detail: "User not found".to_string(),
/// };
/// ```
#[derive(Debug)]
pub struct ApiError {
    /// The HTTP status code for this error
    pub status: StatusCode,
    /// A short, human-readable summary of the error
    pub title: String,
    /// A detailed explanation of the error
    pub detail: String,
}

/// The JSON structure used in error responses.
#[derive(Serialize)]
struct ApiErrorResponse {
    status: u16,
    title: String,
    detail: String,
}

/// Converts from `ApiError` to an HTTP `Response`.
///
/// This implementation allows `ApiError` to be used as a return type in Axum handlers.
/// The error is serialized as JSON with the status code, title, and detail fields.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(ApiErrorResponse {
            status: self.status.as_u16(),
            title: self.title,
            detail: self.detail,
        });

        (self.status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    #[test]
    fn test_api_error_creation() {
        let err = api_error(StatusCode::BAD_REQUEST, "Bad Request", "Invalid input");
        assert_eq!(err.status, StatusCode::BAD_REQUEST);
        assert_eq!(err.title, "Bad Request");
        assert_eq!(err.detail, "Invalid input");
    }

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

    #[test]
    fn test_result_ext_context_bad_request_on_err() {
        let result: Result<i32> = Err(anyhow!("Original error"));
        let api_result = result.context_bad_request("Bad Request", "Invalid data");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::BAD_REQUEST);
        assert_eq!(err.title, "Bad Request");
        assert_eq!(err.detail, "Invalid data");
    }

    #[test]
    fn test_result_ext_context_bad_request_on_ok() {
        let result: Result<i32> = Ok(42);
        let api_result = result.context_bad_request("Bad Request", "Invalid data");

        assert!(api_result.is_ok());
        assert_eq!(api_result.unwrap(), 42);
    }

    #[test]
    fn test_result_ext_context_unauthenticated() {
        let result: Result<String> = Err(anyhow!("Token missing"));
        let api_result = result.context_unauthenticated("Unauthenticated", "No token");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::UNAUTHORIZED);
        assert_eq!(err.title, "Unauthenticated");
        assert_eq!(err.detail, "No token");
    }

    #[test]
    fn test_result_ext_context_unauthorized() {
        let result: Result<String> = Err(anyhow!("Permission denied"));
        let api_result = result.context_unauthorized("Unauthorized", "Insufficient permissions");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::FORBIDDEN);
        assert_eq!(err.title, "Unauthorized");
        assert_eq!(err.detail, "Insufficient permissions");
    }

    #[test]
    fn test_result_ext_context_not_found() {
        let result: Result<String> = Err(anyhow!("Resource missing"));
        let api_result = result.context_not_found("Not Found", "User not found");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::NOT_FOUND);
        assert_eq!(err.title, "Not Found");
        assert_eq!(err.detail, "User not found");
    }

    #[test]
    fn test_result_ext_context_internal() {
        let result: Result<String> = Err(anyhow!("Database error"));
        let api_result = result.context_internal("Internal Error", "Database failed");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(err.title, "Internal Error");
        assert_eq!(err.detail, "Database failed");
    }

    #[test]
    fn test_result_ext_context_status() {
        let result: Result<String> = Err(anyhow!("Custom error"));
        let api_result = result.context_status(StatusCode::CONFLICT, "Conflict", "Duplicate entry");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::CONFLICT);
        assert_eq!(err.title, "Conflict");
        assert_eq!(err.detail, "Duplicate entry");
    }

    #[test]
    fn test_option_ext_ok_or_bad_request_on_none() {
        let option: Option<i32> = None;
        let api_result = option.ok_or_bad_request("Bad Request", "Value is required");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::BAD_REQUEST);
        assert_eq!(err.title, "Bad Request");
        assert_eq!(err.detail, "Value is required");
    }

    #[test]
    fn test_option_ext_ok_or_bad_request_on_some() {
        let option: Option<i32> = Some(42);
        let api_result = option.ok_or_bad_request("Bad Request", "Value is required");

        assert!(api_result.is_ok());
        assert_eq!(api_result.unwrap(), 42);
    }

    #[test]
    fn test_option_ext_ok_or_unauthenticated() {
        let option: Option<String> = None;
        let api_result = option.ok_or_unauthenticated("Unauthenticated", "Token missing");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::UNAUTHORIZED);
        assert_eq!(err.title, "Unauthenticated");
        assert_eq!(err.detail, "Token missing");
    }

    #[test]
    fn test_option_ext_ok_or_unauthorized() {
        let option: Option<String> = None;
        let api_result = option.ok_or_unauthorized("Unauthorized", "No access");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::FORBIDDEN);
        assert_eq!(err.title, "Unauthorized");
        assert_eq!(err.detail, "No access");
    }

    #[test]
    fn test_option_ext_ok_or_not_found() {
        let option: Option<String> = None;
        let api_result = option.ok_or_not_found("Not Found", "Resource missing");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::NOT_FOUND);
        assert_eq!(err.title, "Not Found");
        assert_eq!(err.detail, "Resource missing");
    }

    #[test]
    fn test_option_ext_ok_or_internal() {
        let option: Option<String> = None;
        let api_result = option.ok_or_internal("Internal Error", "Config missing");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(err.title, "Internal Error");
        assert_eq!(err.detail, "Config missing");
    }

    #[test]
    fn test_option_ext_ok_or_status() {
        let option: Option<String> = None;
        let api_result = option.ok_or_status(StatusCode::CONFLICT, "Conflict", "Already exists");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::CONFLICT);
        assert_eq!(err.title, "Conflict");
        assert_eq!(err.detail, "Already exists");
    }

    #[test]
    fn test_into_api_error_from_anyhow() {
        let anyhow_err = anyhow!("Something went wrong");
        let api_err: ApiError = anyhow_err.into();

        assert_eq!(api_err.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(api_err.title, "Internal Error");
        assert_eq!(api_err.detail, "Something went wrong");
    }

    #[test]
    fn test_into_api_error_context_bad_request() {
        let anyhow_err = anyhow!("Invalid input");
        let api_err = anyhow_err.context_bad_request("Bad Request", "Field validation failed");

        assert_eq!(api_err.status, StatusCode::BAD_REQUEST);
        assert_eq!(api_err.title, "Bad Request");
        assert_eq!(api_err.detail, "Field validation failed");
    }

    #[test]
    fn test_into_response() {
        let api_err = bad_request("Bad Request", "Invalid data");
        let response = api_err.into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_chaining_result_operations() {
        fn get_value() -> Result<i32> {
            Err(anyhow!("Failed to get value"))
        }

        let result = get_value().context_bad_request("Bad Request", "Could not retrieve value");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().status, StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_chaining_option_operations() {
        fn get_value() -> Option<i32> {
            None
        }

        let result = get_value().ok_or_not_found("Not Found", "Value does not exist");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().status, StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_question_mark_operator_with_result() {
        fn helper() -> ApiResult<i32> {
            let value: Result<i32> = Err(anyhow!("error"));
            value.context_bad_request("Bad Request", "Invalid")?;
            Ok(42)
        }

        let result = helper();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().status, StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_question_mark_operator_with_option() {
        fn helper() -> ApiResult<i32> {
            let value: Option<i32> = None;
            value.ok_or_not_found("Not Found", "Missing")?;
            Ok(42)
        }

        let result = helper();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().status, StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_api_result_type_alias() {
        fn returns_api_result() -> ApiResult<String> {
            Ok("success".to_string())
        }

        let result = returns_api_result();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }
}
