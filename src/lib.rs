#![doc = include_str!("../README.md")]

mod error;
mod extensions;
mod helpers;
mod hook;

pub use error::{ApiError, ApiErrorBuilder};
pub use extensions::{IntoApiError, OptionExt, ResultExt};
pub use helpers::{bad_request, internal_error, not_found, unauthenticated, unauthorized};
pub use hook::set_error_hook;

use anyhow::Result;

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
