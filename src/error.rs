use crate::{hook::invoke_hook, middleware::EnrichmentContext};
use anyhow::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::Value;
use std::sync::atomic::{AtomicBool, Ordering};

/// Global flag to control whether error details should be exposed in API responses.
/// This can be set programmatically or via the `AXUM_ANYHOW_EXPOSE_ERRORS` environment variable.
static EXPOSE_ERRORS: AtomicBool = AtomicBool::new(false);

/// Sets whether error details should be exposed in API responses.
///
/// When enabled, the actual error message will be included in the `detail` field
/// of error responses. This is useful for development but should be disabled in production.
///
/// # Example
///
/// ```rust
/// use axum_anyhow::set_expose_errors;
///
/// // Enable for development
/// set_expose_errors(true);
///
/// // Disable for production
/// set_expose_errors(false);
/// ```
pub fn set_expose_errors(expose: bool) {
    EXPOSE_ERRORS.store(expose, Ordering::Relaxed);
}

/// Returns whether error details are currently being exposed.
///
/// This checks both the programmatic setting and the `AXUM_ANYHOW_EXPOSE_ERRORS`
/// environment variable. The programmatic setting takes precedence.
///
/// # Example
///
/// ```rust
/// use axum_anyhow::{set_expose_errors, is_expose_errors_enabled};
///
/// set_expose_errors(true);
/// assert!(is_expose_errors_enabled());
/// ```
pub fn is_expose_errors_enabled() -> bool {
    // Check programmatic setting first
    if EXPOSE_ERRORS.load(Ordering::Relaxed) {
        return true;
    }

    // Fall back to environment variable
    std::env::var("AXUM_ANYHOW_EXPOSE_ERRORS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
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
///   "detail": "The requested resource does not exist",
///   "meta": {
///     "request_id": "abc-123"
///   }
/// }
/// ```
///
/// # Example
///
/// ```rust
/// use axum::http::StatusCode;
/// use axum_anyhow::ApiError;
/// use serde_json::json;
///
/// let error = ApiError::builder()
///     .status(StatusCode::NOT_FOUND)
///     .title("Not Found")
///     .detail("User not found")
///     .meta(json!({
///         "request_id": "abc-123"
///     }))
///     .build();
/// ```
#[derive(Debug)]
pub struct ApiError {
    /// The HTTP status code for this error
    status: StatusCode,
    /// A short, human-readable summary of the error
    title: String,
    /// A detailed explanation of the error
    detail: String,
    /// Optional metadata that can be included in the error response
    meta: Option<Value>,
    /// The underlying error that caused this API error
    error: Option<Error>,
}

impl ApiError {
    /// Gets the HTTP status code
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Gets the error title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Gets the error detail
    pub fn detail(&self) -> &str {
        &self.detail
    }

    /// Gets the metadata, if any
    pub fn meta(&self) -> Option<&Value> {
        self.meta.as_ref()
    }

    /// Gets the underlying error, if any
    pub fn error(&self) -> Option<&Error> {
        self.error.as_ref()
    }

    /// Creates a new builder for constructing an `ApiError`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum::http::StatusCode;
    /// use axum_anyhow::ApiError;
    /// use anyhow::anyhow;
    ///
    /// let error = ApiError::builder()
    ///     .status(StatusCode::BAD_REQUEST)
    ///     .title("Validation Error")
    ///     .detail("Email address is required")
    ///     .build();
    /// ```
    pub fn builder() -> ApiErrorBuilder {
        ApiErrorBuilder::default()
    }
    /// Converts this `ApiError` into an `anyhow::Error`.
    ///
    /// If the `ApiError` contains an underlying error, it will be returned with
    /// additional context from the title and detail. Otherwise, a new error is
    /// created from the title and detail.
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum::http::StatusCode;
    /// use axum_anyhow::ApiError;
    /// use anyhow::anyhow;
    ///
    /// let api_error = ApiError::builder()
    ///     .status(StatusCode::INTERNAL_SERVER_ERROR)
    ///     .title("Database Error")
    ///     .detail("Failed to connect")
    ///     .error(anyhow!("Connection timeout"))
    ///     .build();
    ///
    /// let anyhow_error = api_error.into_error();
    /// ```
    pub fn into_error(self) -> Error {
        if let Some(error) = self.error {
            error.context(format!("{}: {}", self.title, self.detail))
        } else {
            anyhow::anyhow!("{}: {}", self.title, self.detail)
        }
    }
}

impl Default for ApiError {
    /// Creates a default `ApiError` with:
    /// - `status`: `StatusCode::INTERNAL_SERVER_ERROR`
    /// - `title`: `"Internal Error"`
    /// - `detail`: `"Something went wrong"`
    /// - `error`: `None`
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum::http::StatusCode;
    /// use axum_anyhow::ApiError;
    ///
    /// let error = ApiError::default();
    /// assert_eq!(error.status(), StatusCode::INTERNAL_SERVER_ERROR);
    /// assert_eq!(error.title(), "Internal Error");
    /// assert_eq!(error.detail(), "Something went wrong");
    /// assert!(error.error().is_none());
    /// ```
    fn default() -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            title: "Internal Error".to_string(),
            detail: "Something went wrong".to_string(),
            meta: None,
            error: None,
        }
    }
}

/// Converts from `anyhow::Error` to `ApiError`.
///
/// By default, all errors are converted to 500 Internal Server Error responses.
/// Use the extension traits to specify different status codes.
///
/// Set the `AXUM_ANYHOW_EXPOSE_ERRORS` environment variable or use `set_expose_errors(true)`
/// to expose the actual error message in the detail field (useful for development).
impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        let error = err.into();
        let should_expose = is_expose_errors_enabled();

        let mut builder = ApiError::builder();
        if should_expose {
            builder = builder.detail(error.to_string());
        }
        builder.error(error).build()
    }
}

/// The JSON structure used in error responses.
#[derive(Serialize)]
struct ApiErrorResponse {
    status: u16,
    title: String,
    detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<Value>,
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
            meta: self.meta,
        });

        (self.status, body).into_response()
    }
}

/// A builder for constructing `ApiError` instances.
///
/// This builder provides a fluent interface for creating `ApiError` instances with
/// optional fields. The `status`, `title`, and `detail` fields are required and must
/// be set before calling `build()`.
///
/// # Example
///
/// ```rust
/// use axum::http::StatusCode;
/// use axum_anyhow::ApiError;
/// use anyhow::anyhow;
///
/// let error = ApiError::builder()
///     .status(StatusCode::INTERNAL_SERVER_ERROR)
///     .title("Database Error")
///     .detail("Failed to connect to the database")
///     .error(anyhow!("Connection timeout"))
///     .build();
/// ```
#[derive(Default)]
pub struct ApiErrorBuilder {
    status: Option<StatusCode>,
    title: Option<String>,
    detail: Option<String>,
    meta: Option<Value>,
    error: Option<Error>,
}

impl Clone for ApiErrorBuilder {
    fn clone(&self) -> Self {
        Self {
            status: self.status,
            title: self.title.clone(),
            detail: self.detail.clone(),
            meta: self.meta.clone(),
            // anyhow::Error doesn't implement Clone, so we skip it
            error: None,
        }
    }
}

impl ApiErrorBuilder {
    /// Sets the HTTP status code for the error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum::http::StatusCode;
    /// use axum_anyhow::ApiError;
    ///
    /// let error = ApiError::builder()
    ///     .status(StatusCode::NOT_FOUND)
    ///     .title("Not Found")
    ///     .detail("Resource not found")
    ///     .build();
    /// ```
    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the title for the error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum::http::StatusCode;
    /// use axum_anyhow::ApiError;
    ///
    /// let error = ApiError::builder()
    ///     .status(StatusCode::BAD_REQUEST)
    ///     .title("Invalid Input")
    ///     .detail("The provided email is invalid")
    ///     .build();
    /// ```
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the detail message for the error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum::http::StatusCode;
    /// use axum_anyhow::ApiError;
    ///
    /// let error = ApiError::builder()
    ///     .status(StatusCode::FORBIDDEN)
    ///     .title("Access Denied")
    ///     .detail("You do not have permission to access this resource")
    ///     .build();
    /// ```
    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Sets the underlying error that caused this API error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum::http::StatusCode;
    /// use axum_anyhow::ApiError;
    /// use anyhow::anyhow;
    ///
    /// let error = ApiError::builder()
    ///     .status(StatusCode::INTERNAL_SERVER_ERROR)
    ///     .title("Database Error")
    ///     .detail("Failed to execute query")
    ///     .error(anyhow!("Connection pool exhausted"))
    ///     .build();
    ///
    /// assert_eq!(error.status(), StatusCode::INTERNAL_SERVER_ERROR);
    /// assert_eq!(error.title(), "Database Error");
    /// assert_eq!(error.detail(), "Failed to execute query");
    /// assert_eq!(error.error().unwrap().to_string(), "Connection pool exhausted");
    /// ```
    pub fn error(mut self, error: impl Into<Error>) -> Self {
        self.error = Some(error.into());
        self
    }

    /// Sets custom metadata for the error.
    ///
    /// This metadata will be included in the JSON response under the `meta` field.
    /// It can contain any JSON-serializable data, such as request IDs, trace information,
    /// or other contextual data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum::http::StatusCode;
    /// use axum_anyhow::ApiError;
    /// use serde_json::json;
    ///
    /// let error = ApiError::builder()
    ///     .status(StatusCode::NOT_FOUND)
    ///     .title("Not Found")
    ///     .detail("User not found")
    ///     .meta(json!({"request_id": "abc-123", "timestamp": 1234567890}))
    ///     .build();
    ///
    /// assert!(error.meta().is_some());
    /// ```
    pub fn meta(mut self, meta: Value) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Builds the `ApiError` instance.
    ///
    /// If `status`, `title`, or `detail` have not been set, they will default to:
    /// - `status`: `StatusCode::INTERNAL_SERVER_ERROR`
    /// - `title`: `"Internal Error"`
    /// - `detail`: `"Something went wrong"`
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum::http::StatusCode;
    /// use axum_anyhow::ApiError;
    ///
    /// let error = ApiError::builder()
    ///     .status(StatusCode::BAD_REQUEST)
    ///     .title("Bad Request")
    ///     .detail("Invalid request parameters")
    ///     .build();
    ///
    /// assert_eq!(error.status(), StatusCode::BAD_REQUEST);
    /// assert_eq!(error.title(), "Bad Request");
    /// assert_eq!(error.detail(), "Invalid request parameters");
    ///
    /// // Using defaults
    /// let default_error = ApiError::builder().build();
    /// assert_eq!(default_error.status(), StatusCode::INTERNAL_SERVER_ERROR);
    /// assert_eq!(default_error.title(), "Internal Error");
    /// assert_eq!(default_error.detail(), "Something went wrong");
    /// ```
    pub fn build(mut self) -> ApiError {
        // Invoke enricher if middleware is enabled and request context is available
        self = EnrichmentContext::invoke(self);

        let error = ApiError {
            status: self.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            title: self.title.unwrap_or_else(|| "Internal Error".to_string()),
            detail: self
                .detail
                .unwrap_or_else(|| "Something went wrong".to_string()),
            meta: self.meta,
            error: self.error,
        };

        invoke_hook(&error);
        error
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use http_body_util::BodyExt;
    use serde_json::Value;
    use serial_test::serial;

    #[test]
    fn test_into_api_error_from_anyhow() {
        let anyhow_err = anyhow!("Something went wrong");
        let api_err: ApiError = anyhow_err.into();

        assert_eq!(api_err.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(api_err.title, "Internal Error");
        assert_eq!(api_err.detail, "Something went wrong");
    }

    #[test]
    fn test_api_error_builder() {
        let error = ApiError::builder()
            .status(StatusCode::BAD_REQUEST)
            .title("Validation Error")
            .detail("Email is required")
            .build();

        assert_eq!(error.status, StatusCode::BAD_REQUEST);
        assert_eq!(error.title, "Validation Error");
        assert_eq!(error.detail, "Email is required");
        assert!(error.error.is_none());
    }

    #[test]
    fn test_api_error_builder_with_error() {
        let underlying_error = anyhow!("Database connection failed");
        let error = ApiError::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .title("Database Error")
            .detail("Could not connect to the database")
            .error(underlying_error)
            .build();

        assert_eq!(error.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(error.title, "Database Error");
        assert_eq!(error.detail, "Could not connect to the database");
        assert!(error.error.is_some());
    }

    #[test]
    fn test_api_error_builder_with_string_conversions() {
        let error = ApiError::builder()
            .status(StatusCode::NOT_FOUND)
            .title("Not Found".to_string())
            .detail("Resource not found".to_string())
            .build();

        assert_eq!(error.status, StatusCode::NOT_FOUND);
        assert_eq!(error.title, "Not Found");
        assert_eq!(error.detail, "Resource not found");
    }

    #[test]
    fn test_api_error_builder_missing_status() {
        let error = ApiError::builder()
            .title("Error")
            .detail("Something went wrong")
            .build();

        assert_eq!(error.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(error.title, "Error");
        assert_eq!(error.detail, "Something went wrong");
    }

    #[test]
    fn test_api_error_builder_missing_title() {
        let error = ApiError::builder()
            .status(StatusCode::BAD_REQUEST)
            .detail("Something went wrong")
            .build();

        assert_eq!(error.status, StatusCode::BAD_REQUEST);
        assert_eq!(error.title, "Internal Error");
        assert_eq!(error.detail, "Something went wrong");
    }

    #[test]
    fn test_api_error_builder_missing_detail() {
        let error = ApiError::builder()
            .status(StatusCode::BAD_REQUEST)
            .title("Error")
            .build();

        assert_eq!(error.status, StatusCode::BAD_REQUEST);
        assert_eq!(error.title, "Error");
        assert_eq!(error.detail, "Something went wrong");
    }

    #[test]
    fn test_api_error_builder_all_defaults() {
        let error = ApiError::builder().build();

        assert_eq!(error.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(error.title, "Internal Error");
        assert_eq!(error.detail, "Something went wrong");
        assert!(error.error.is_none());
    }

    #[test]
    fn test_api_error_builder_fluent_interface() {
        let error = ApiError::builder()
            .status(StatusCode::CONFLICT)
            .title("Conflict")
            .detail("User already exists")
            .error(anyhow!("Duplicate email"))
            .build();

        assert_eq!(error.status, StatusCode::CONFLICT);
        assert_eq!(error.title, "Conflict");
        assert_eq!(error.detail, "User already exists");
        assert!(error.error.is_some());
    }

    #[test]
    fn test_api_error_default() {
        let error = ApiError::default();

        assert_eq!(error.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(error.title, "Internal Error");
        assert_eq!(error.detail, "Something went wrong");
        assert!(error.error.is_none());
    }

    #[test]
    fn test_anyhow_error_coerced_to_api_error_has_defaults() {
        set_expose_errors(false);

        let anyhow_err = anyhow!("Some error occurred");
        let api_err: ApiError = anyhow_err.into();

        assert_eq!(api_err.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(api_err.title, "Internal Error");
        assert_eq!(api_err.detail, "Something went wrong");
        assert!(api_err.error.is_some());
    }

    #[test]
    fn test_api_error_default_matches_builder_defaults() {
        let from_default = ApiError::default();
        let from_builder = ApiError::builder().build();

        assert_eq!(from_default.status, from_builder.status);
        assert_eq!(from_default.title, from_builder.title);
        assert_eq!(from_default.detail, from_builder.detail);
        assert!(from_default.error.is_none());
        assert!(from_builder.error.is_none());
    }

    #[tokio::test]
    async fn test_into_response_status() {
        let api_err = ApiError::builder()
            .status(StatusCode::BAD_REQUEST)
            .title("Bad Request")
            .detail("Invalid data")
            .build();

        let response = api_err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_into_response_json_structure() {
        let api_err = ApiError::builder()
            .status(StatusCode::NOT_FOUND)
            .title("Not Found")
            .detail("Resource does not exist")
            .build();

        let response = api_err.into_response();

        // Verify status
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        // Verify JSON body structure
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["status"], 404);
        assert_eq!(json["title"], "Not Found");
        assert_eq!(json["detail"], "Resource does not exist");
    }

    #[test]
    fn test_into_error_with_underlying_error() {
        let underlying = anyhow!("Connection timeout");
        let api_error = ApiError::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .title("Database Error")
            .detail("Failed to connect")
            .error(underlying)
            .build();

        let anyhow_error = api_error.into_error();
        let error_msg = format!("{:#}", anyhow_error);

        // Should contain both the context and the underlying error
        assert!(error_msg.contains("Database Error: Failed to connect"));
        assert!(error_msg.contains("Connection timeout"));
    }

    #[test]
    fn test_into_error_without_underlying_error() {
        let api_error = ApiError::builder()
            .status(StatusCode::BAD_REQUEST)
            .title("Validation Error")
            .detail("Email is required")
            .build();

        let anyhow_error = api_error.into_error();
        let error_msg = anyhow_error.to_string();

        assert_eq!(error_msg, "Validation Error: Email is required");
    }

    #[test]
    #[serial]
    fn test_expose_error_details_enabled() {
        set_expose_errors(true);

        let anyhow_err = anyhow!("Database connection failed");
        let api_err: ApiError = anyhow_err.into();

        assert_eq!(api_err.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(api_err.title, "Internal Error");
        assert_eq!(api_err.detail, "Database connection failed");
        assert!(api_err.error.is_some());

        set_expose_errors(false);
    }

    #[test]
    #[serial]
    fn test_expose_error_details_disabled() {
        set_expose_errors(false);

        let anyhow_err = anyhow!("Database connection failed");
        let api_err: ApiError = anyhow_err.into();

        assert_eq!(api_err.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(api_err.title, "Internal Error");
        assert_eq!(api_err.detail, "Something went wrong");
        assert!(api_err.error.is_some());
    }

    #[test]
    #[serial]
    fn test_expose_error_details_with_true() {
        set_expose_errors(true);

        let anyhow_err = anyhow!("Connection timeout");
        let api_err: ApiError = anyhow_err.into();

        assert_eq!(api_err.detail, "Connection timeout");

        set_expose_errors(false);
    }

    #[test]
    #[serial]
    fn test_expose_error_via_env_var() {
        // Ensure programmatic setting is disabled
        set_expose_errors(false);

        // Set environment variable
        unsafe {
            std::env::set_var("AXUM_ANYHOW_EXPOSE_ERRORS", "1");
        }

        let anyhow_err = anyhow!("Environment variable test");
        let api_err: ApiError = anyhow_err.into();

        // Should expose details via env var
        assert_eq!(api_err.detail, "Environment variable test");

        // Clean up
        unsafe {
            std::env::remove_var("AXUM_ANYHOW_EXPOSE_ERRORS");
        }
    }

    #[test]
    #[serial]
    fn test_programmatic_setting_overrides_env_var() {
        // Set environment variable to disabled
        unsafe {
            std::env::set_var("AXUM_ANYHOW_EXPOSE_ERRORS", "0");
        }

        // But enable programmatically (should take precedence)
        set_expose_errors(true);

        let anyhow_err = anyhow!("Programmatic override test");
        let api_err: ApiError = anyhow_err.into();

        // Should expose details because programmatic setting takes precedence
        assert_eq!(api_err.detail, "Programmatic override test");

        // Clean up
        set_expose_errors(false);
        unsafe {
            std::env::remove_var("AXUM_ANYHOW_EXPOSE_ERRORS");
        }
    }

    #[test]
    fn test_api_error_with_meta() {
        use serde_json::json;

        let error = ApiError::builder()
            .status(StatusCode::NOT_FOUND)
            .title("Not Found")
            .detail("User not found")
            .meta(json!({"request_id": "abc-123", "timestamp": 1234567890}))
            .build();

        assert_eq!(error.status, StatusCode::NOT_FOUND);
        assert_eq!(error.title, "Not Found");
        assert_eq!(error.detail, "User not found");
        assert!(error.meta.is_some());

        let meta = error.meta.unwrap();
        assert_eq!(meta["request_id"], "abc-123");
        assert_eq!(meta["timestamp"], 1234567890);
    }

    #[test]
    fn test_api_error_without_meta() {
        let error = ApiError::builder()
            .status(StatusCode::BAD_REQUEST)
            .title("Bad Request")
            .detail("Invalid input")
            .build();

        assert!(error.meta.is_none());
    }

    #[test]
    fn test_api_error_default_has_none_meta() {
        let error = ApiError::default();
        assert!(error.meta.is_none());
    }

    #[tokio::test]
    async fn test_into_response_with_meta() {
        use serde_json::json;

        let api_err = ApiError::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .title("Server Error")
            .detail("Something went wrong")
            .meta(json!({"trace_id": "xyz-789"}))
            .build();

        let response = api_err.into_response();

        // Verify status
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        // Verify JSON body includes meta
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["status"], 500);
        assert_eq!(json["title"], "Server Error");
        assert_eq!(json["detail"], "Something went wrong");
        assert!(json["meta"].is_object());
        assert_eq!(json["meta"]["trace_id"], "xyz-789");
    }

    #[tokio::test]
    async fn test_into_response_without_meta() {
        let api_err = ApiError::builder()
            .status(StatusCode::NOT_FOUND)
            .title("Not Found")
            .detail("Resource not found")
            .build();

        let response = api_err.into_response();

        // Verify JSON body does not include meta field
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let json: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(json["status"], 404);
        assert_eq!(json["title"], "Not Found");
        assert_eq!(json["detail"], "Resource not found");
        // Meta should be omitted from JSON when None (due to skip_serializing_if)
        assert!(json.get("meta").is_none());
    }

    #[test]
    fn test_api_error_builder_fluent_with_meta() {
        use serde_json::json;

        let error = ApiError::builder()
            .status(StatusCode::CONFLICT)
            .title("Conflict")
            .detail("Resource already exists")
            .meta(json!({"duplicate_field": "email", "value": "test@example.com"}))
            .error(anyhow!("Unique constraint violation"))
            .build();

        assert_eq!(error.status, StatusCode::CONFLICT);
        assert_eq!(error.title, "Conflict");
        assert_eq!(error.detail, "Resource already exists");
        assert!(error.error.is_some());
        assert!(error.meta.is_some());

        let meta = error.meta.unwrap();
        assert_eq!(meta["duplicate_field"], "email");
        assert_eq!(meta["value"], "test@example.com");
    }
}
