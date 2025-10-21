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
mod helpers;

pub use error::{ApiError, ApiErrorBuilder};
pub use extensions::{IntoApiError, OptionExt, ResultExt};
pub use helpers::{bad_request, internal_error, not_found, unauthenticated, unauthorized};

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
