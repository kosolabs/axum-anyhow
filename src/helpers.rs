use crate::ApiError;
use axum::http::StatusCode;

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
/// use axum::http::StatusCode;
///
/// let error = bad_request("Invalid Input", "Email format is invalid");
/// assert_eq!(error.status, StatusCode::BAD_REQUEST);
/// assert_eq!(error.title, "Invalid Input");
/// assert_eq!(error.detail, "Email format is invalid");
/// ```
pub fn bad_request(title: &str, detail: &str) -> ApiError {
    ApiError::builder()
        .status(StatusCode::BAD_REQUEST)
        .title(title)
        .detail(detail)
        .build()
}

/// Creates a 401 Unauthorized error (missing or invalid credentials).
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
/// use axum::http::StatusCode;
///
/// let error = unauthorized("Unauthorized", "No valid authentication token provided");
/// assert_eq!(error.status, StatusCode::UNAUTHORIZED);
/// assert_eq!(error.title, "Unauthorized");
/// assert_eq!(error.detail, "No valid authentication token provided");
/// ```
pub fn unauthorized(title: &str, detail: &str) -> ApiError {
    ApiError::builder()
        .status(StatusCode::UNAUTHORIZED)
        .title(title)
        .detail(detail)
        .build()
}

/// Creates a 403 Forbidden error (authenticated but lacks permissions).
///
/// # Arguments
///
/// * `title` - A short, human-readable summary of the error
/// * `detail` - A detailed explanation of the error
///
/// # Example
///
/// ```rust
/// use axum_anyhow::forbidden;
/// use axum::http::StatusCode;
///
/// let error = forbidden("Forbidden", "You do not have permission to access this resource");
/// assert_eq!(error.status, StatusCode::FORBIDDEN);
/// assert_eq!(error.title, "Forbidden");
/// assert_eq!(error.detail, "You do not have permission to access this resource");
/// ```
pub fn forbidden(title: &str, detail: &str) -> ApiError {
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
/// use axum::http::StatusCode;
///
/// let error = not_found("Not Found", "The requested user does not exist");
/// assert_eq!(error.status, StatusCode::NOT_FOUND);
/// assert_eq!(error.title, "Not Found");
/// assert_eq!(error.detail, "The requested user does not exist");
/// ```
pub fn not_found(title: &str, detail: &str) -> ApiError {
    ApiError::builder()
        .status(StatusCode::NOT_FOUND)
        .title(title)
        .detail(detail)
        .build()
}

/// Creates a 405 Method Not Allowed error.
///
/// # Arguments
///
/// * `title` - A short, human-readable summary of the error
/// * `detail` - A detailed explanation of the error
///
/// # Example
///
/// ```rust
/// use axum_anyhow::method_not_allowed;
/// use axum::http::StatusCode;
///
/// let error = method_not_allowed("Method Not Allowed", "POST method is not supported for this endpoint");
/// assert_eq!(error.status, StatusCode::METHOD_NOT_ALLOWED);
/// assert_eq!(error.title, "Method Not Allowed");
/// assert_eq!(error.detail, "POST method is not supported for this endpoint");
/// ```
pub fn method_not_allowed(title: &str, detail: &str) -> ApiError {
    ApiError::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .title(title)
        .detail(detail)
        .build()
}

/// Creates a 409 Conflict error.
///
/// # Arguments
///
/// * `title` - A short, human-readable summary of the error
/// * `detail` - A detailed explanation of the error
///
/// # Example
///
/// ```rust
/// use axum_anyhow::conflict;
/// use axum::http::StatusCode;
///
/// let error = conflict("Conflict", "A user with this email already exists");
/// assert_eq!(error.status, StatusCode::CONFLICT);
/// assert_eq!(error.title, "Conflict");
/// assert_eq!(error.detail, "A user with this email already exists");
/// ```
pub fn conflict(title: &str, detail: &str) -> ApiError {
    ApiError::builder()
        .status(StatusCode::CONFLICT)
        .title(title)
        .detail(detail)
        .build()
}

/// Creates a 422 Unprocessable Entity error.
///
/// # Arguments
///
/// * `title` - A short, human-readable summary of the error
/// * `detail` - A detailed explanation of the error
///
/// # Example
///
/// ```rust
/// use axum_anyhow::unprocessable_entity;
/// use axum::http::StatusCode;
///
/// let error = unprocessable_entity("Validation Failed", "Password must be at least 8 characters");
/// assert_eq!(error.status, StatusCode::UNPROCESSABLE_ENTITY);
/// assert_eq!(error.title, "Validation Failed");
/// assert_eq!(error.detail, "Password must be at least 8 characters");
/// ```
pub fn unprocessable_entity(title: &str, detail: &str) -> ApiError {
    ApiError::builder()
        .status(StatusCode::UNPROCESSABLE_ENTITY)
        .title(title)
        .detail(detail)
        .build()
}

/// Creates a 429 Too Many Requests error.
///
/// # Arguments
///
/// * `title` - A short, human-readable summary of the error
/// * `detail` - A detailed explanation of the error
///
/// # Example
///
/// ```rust
/// use axum_anyhow::too_many_requests;
/// use axum::http::StatusCode;
///
/// let error = too_many_requests("Too Many Requests", "Rate limit exceeded. Please try again later");
/// assert_eq!(error.status, StatusCode::TOO_MANY_REQUESTS);
/// assert_eq!(error.title, "Too Many Requests");
/// assert_eq!(error.detail, "Rate limit exceeded. Please try again later");
/// ```
pub fn too_many_requests(title: &str, detail: &str) -> ApiError {
    ApiError::builder()
        .status(StatusCode::TOO_MANY_REQUESTS)
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
/// use axum::http::StatusCode;
///
/// let error = internal_error("Internal Error", "Database connection failed");
/// assert_eq!(error.status, StatusCode::INTERNAL_SERVER_ERROR);
/// assert_eq!(error.title, "Internal Error");
/// assert_eq!(error.detail, "Database connection failed");
/// ```
pub fn internal_error(title: &str, detail: &str) -> ApiError {
    ApiError::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .title(title)
        .detail(detail)
        .build()
}

/// Creates a 502 Bad Gateway error.
///
/// # Arguments
///
/// * `title` - A short, human-readable summary of the error
/// * `detail` - A detailed explanation of the error
///
/// # Example
///
/// ```rust
/// use axum_anyhow::bad_gateway;
/// use axum::http::StatusCode;
///
/// let error = bad_gateway("Bad Gateway", "Upstream service returned an invalid response");
/// assert_eq!(error.status, StatusCode::BAD_GATEWAY);
/// assert_eq!(error.title, "Bad Gateway");
/// assert_eq!(error.detail, "Upstream service returned an invalid response");
/// ```
pub fn bad_gateway(title: &str, detail: &str) -> ApiError {
    ApiError::builder()
        .status(StatusCode::BAD_GATEWAY)
        .title(title)
        .detail(detail)
        .build()
}

/// Creates a 503 Service Unavailable error.
///
/// # Arguments
///
/// * `title` - A short, human-readable summary of the error
/// * `detail` - A detailed explanation of the error
///
/// # Example
///
/// ```rust
/// use axum_anyhow::service_unavailable;
/// use axum::http::StatusCode;
///
/// let error = service_unavailable("Service Unavailable", "Database is currently under maintenance");
/// assert_eq!(error.status, StatusCode::SERVICE_UNAVAILABLE);
/// assert_eq!(error.title, "Service Unavailable");
/// assert_eq!(error.detail, "Database is currently under maintenance");
/// ```
pub fn service_unavailable(title: &str, detail: &str) -> ApiError {
    ApiError::builder()
        .status(StatusCode::SERVICE_UNAVAILABLE)
        .title(title)
        .detail(detail)
        .build()
}

/// Creates a 504 Gateway Timeout error.
///
/// # Arguments
///
/// * `title` - A short, human-readable summary of the error
/// * `detail` - A detailed explanation of the error
///
/// # Example
///
/// ```rust
/// use axum_anyhow::gateway_timeout;
/// use axum::http::StatusCode;
///
/// let error = gateway_timeout("Gateway Timeout", "Upstream service did not respond in time");
/// assert_eq!(error.status, StatusCode::GATEWAY_TIMEOUT);
/// assert_eq!(error.title, "Gateway Timeout");
/// assert_eq!(error.detail, "Upstream service did not respond in time");
/// ```
pub fn gateway_timeout(title: &str, detail: &str) -> ApiError {
    ApiError::builder()
        .status(StatusCode::GATEWAY_TIMEOUT)
        .title(title)
        .detail(detail)
        .build()
}
