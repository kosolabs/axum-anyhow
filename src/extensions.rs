use crate::{ApiError, ApiResult};
use anyhow::Result;
use axum::http::StatusCode;

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

    /// Converts an error to a 401 Unauthorized error (missing or invalid credentials).
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_unauthorized(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts an error to a 403 Forbidden error (authenticated but lacks permissions).
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_forbidden(self, title: &str, detail: &str) -> ApiResult<T>;

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

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: IntoApiError,
{
    fn context_status(self, status: StatusCode, title: &str, detail: &str) -> ApiResult<T> {
        self.map_err(|err| err.context_status(status, title, detail))
    }

    fn context_bad_request(self, title: &str, detail: &str) -> ApiResult<T> {
        self.map_err(|err| err.context_bad_request(title, detail))
    }

    fn context_unauthorized(self, title: &str, detail: &str) -> ApiResult<T> {
        self.map_err(|err| err.context_unauthorized(title, detail))
    }

    fn context_forbidden(self, title: &str, detail: &str) -> ApiResult<T> {
        self.map_err(|err| err.context_forbidden(title, detail))
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
///         .context_not_found("User Not Found", "No user with that ID exists")?;
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
    fn context_status(self, status: StatusCode, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 400 Bad Request error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_bad_request(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 401 Unauthorized error (missing or invalid credentials).
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_unauthorized(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 403 Forbidden error (authenticated but lacks permissions).
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_forbidden(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 404 Not Found error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_not_found(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 500 Internal Server Error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_internal(self, title: &str, detail: &str) -> ApiResult<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn context_status(self, status: StatusCode, title: &str, detail: &str) -> ApiResult<T> {
        self.ok_or_else(|| {
            ApiError::builder()
                .status(status)
                .title(title)
                .detail(detail)
                .build()
        })
    }

    fn context_bad_request(self, title: &str, detail: &str) -> ApiResult<T> {
        self.context_status(StatusCode::BAD_REQUEST, title, detail)
    }

    fn context_unauthorized(self, title: &str, detail: &str) -> ApiResult<T> {
        self.context_status(StatusCode::UNAUTHORIZED, title, detail)
    }

    fn context_forbidden(self, title: &str, detail: &str) -> ApiResult<T> {
        self.context_status(StatusCode::FORBIDDEN, title, detail)
    }

    fn context_not_found(self, title: &str, detail: &str) -> ApiResult<T> {
        self.context_status(StatusCode::NOT_FOUND, title, detail)
    }

    fn context_internal(self, title: &str, detail: &str) -> ApiResult<T> {
        self.context_status(StatusCode::INTERNAL_SERVER_ERROR, title, detail)
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
pub trait IntoApiError {
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

    /// Converts an error to a 401 Unauthorized error (missing or invalid credentials).
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_unauthorized(self, title: &str, detail: &str) -> ApiError;

    /// Converts an error to a 403 Forbidden error (authenticated but lacks permissions).
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_forbidden(self, title: &str, detail: &str) -> ApiError;

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

impl<E> IntoApiError for E
where
    E: Into<anyhow::Error>,
{
    fn context_status(self, status: StatusCode, title: &str, detail: &str) -> ApiError {
        ApiError::builder()
            .status(status)
            .title(title)
            .detail(detail)
            .error(self)
            .build()
    }

    fn context_bad_request(self, title: &str, detail: &str) -> ApiError {
        self.context_status(StatusCode::BAD_REQUEST, title, detail)
    }

    fn context_unauthorized(self, title: &str, detail: &str) -> ApiError {
        self.context_status(StatusCode::UNAUTHORIZED, title, detail)
    }

    fn context_forbidden(self, title: &str, detail: &str) -> ApiError {
        self.context_status(StatusCode::FORBIDDEN, title, detail)
    }

    fn context_not_found(self, title: &str, detail: &str) -> ApiError {
        self.context_status(StatusCode::NOT_FOUND, title, detail)
    }

    fn context_internal(self, title: &str, detail: &str) -> ApiError {
        self.context_status(StatusCode::INTERNAL_SERVER_ERROR, title, detail)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

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
    fn test_result_ext_context_unauthorized() {
        let result: Result<String> = Err(anyhow!("Token missing"));
        let api_result = result.context_unauthorized("Unauthorized", "No token");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::UNAUTHORIZED);
        assert_eq!(err.title, "Unauthorized");
        assert_eq!(err.detail, "No token");
    }

    #[test]
    fn test_result_ext_context_forbidden() {
        let result: Result<String> = Err(anyhow!("Permission denied"));
        let api_result = result.context_forbidden("Forbidden", "Insufficient permissions");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::FORBIDDEN);
        assert_eq!(err.title, "Forbidden");
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
    fn test_result_ext_with_non_anyhow_error() {
        let result = "not_a_number".parse::<i32>();
        let api_result = result.context_bad_request("Bad Request", "Value must be a number");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::BAD_REQUEST);
        assert_eq!(err.title, "Bad Request");
        assert_eq!(err.detail, "Value must be a number");
    }

    #[test]
    fn test_option_ext_context_bad_request_on_none() {
        let option: Option<i32> = None;
        let api_result = option.context_bad_request("Bad Request", "Value is required");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::BAD_REQUEST);
        assert_eq!(err.title, "Bad Request");
        assert_eq!(err.detail, "Value is required");
    }

    #[test]
    fn test_option_ext_context_bad_request_on_some() {
        let option: Option<i32> = Some(42);
        let api_result = option.context_bad_request("Bad Request", "Value is required");

        assert!(api_result.is_ok());
        assert_eq!(api_result.unwrap(), 42);
    }

    #[test]
    fn test_option_ext_context_unauthorized() {
        let option: Option<String> = None;
        let api_result = option.context_unauthorized("Unauthorized", "Token missing");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::UNAUTHORIZED);
        assert_eq!(err.title, "Unauthorized");
        assert_eq!(err.detail, "Token missing");
    }

    #[test]
    fn test_option_ext_context_forbidden() {
        let option: Option<String> = None;
        let api_result = option.context_forbidden("Forbidden", "No access");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::FORBIDDEN);
        assert_eq!(err.title, "Forbidden");
        assert_eq!(err.detail, "No access");
    }

    #[test]
    fn test_option_ext_context_not_found() {
        let option: Option<String> = None;
        let api_result = option.context_not_found("Not Found", "Resource missing");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::NOT_FOUND);
        assert_eq!(err.title, "Not Found");
        assert_eq!(err.detail, "Resource missing");
    }

    #[test]
    fn test_option_ext_context_internal() {
        let option: Option<String> = None;
        let api_result = option.context_internal("Internal Error", "Config missing");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(err.title, "Internal Error");
        assert_eq!(err.detail, "Config missing");
    }

    #[test]
    fn test_option_ext_context_status() {
        let option: Option<String> = None;
        let api_result = option.context_status(StatusCode::CONFLICT, "Conflict", "Already exists");

        assert!(api_result.is_err());
        let err = api_result.unwrap_err();
        assert_eq!(err.status, StatusCode::CONFLICT);
        assert_eq!(err.title, "Conflict");
        assert_eq!(err.detail, "Already exists");
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

        let result = get_value().context_not_found("Not Found", "Value does not exist");

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
            value.context_not_found("Not Found", "Missing")?;
            Ok(42)
        }

        let result = helper();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().status, StatusCode::NOT_FOUND);
    }
}
