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
///
/// # Sealed Trait
///
/// This trait is sealed and cannot be implemented for types outside this crate.
/// This is intentional to allow adding new methods in the future without breaking changes.
///
/// ```compile_fail
/// use axum_anyhow::ResultExt;
/// use axum::http::StatusCode;
///
/// struct MyError;
///
/// // This will not compile - trait is sealed
/// impl<T> ResultExt<T> for Result<T, MyError> {
///     fn context_status(self, status: StatusCode, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> {
///         todo!()
///     }
///     fn context_bad_request(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_unauthorized(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_forbidden(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_not_found(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_method_not_allowed(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_conflict(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_unprocessable_entity(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_too_many_requests(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_internal(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_bad_gateway(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_service_unavailable(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_gateway_timeout(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
/// }
/// ```
pub trait ResultExt<T>: sealed::SealedResult {
    /// Converts an error to an `ApiError` with a custom status code.
    ///
    /// # Arguments
    ///
    /// * `status` - The HTTP status code to use
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::{anyhow, Result};
    /// use axum_anyhow::{ApiResult, ResultExt};
    /// use axum::http::StatusCode;
    ///
    /// fn make_tea() -> Result<String> {
    ///     Err(anyhow!("I refuse to brew coffee"))
    /// }
    ///
    /// let result: ApiResult<String> = make_tea()
    ///     .context_status(StatusCode::IM_A_TEAPOT, "I'm a teapot", "This server is a teapot, not a coffee maker");
    /// assert!(result.is_err());
    /// let err = result.unwrap_err();
    /// assert_eq!(err.status, StatusCode::IM_A_TEAPOT);
    /// assert_eq!(err.title, "I'm a teapot");
    /// assert_eq!(err.detail, "This server is a teapot, not a coffee maker");
    /// ```
    fn context_status(self, status: StatusCode, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts an error to a 400 Bad Request error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::{anyhow, Result};
    /// use axum_anyhow::{ApiResult, ResultExt};
    /// use axum::http::StatusCode;
    ///
    /// fn parse_age(input: &str) -> Result<u32> {
    ///     input.parse().map_err(|e| anyhow!("Parse error: {}", e))
    /// }
    ///
    /// let result = parse_age("not a number")
    ///     .context_bad_request("Invalid Input", "Age must be a valid number");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::BAD_REQUEST);
    /// ```
    fn context_bad_request(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts an error to a 401 Unauthorized error (missing or invalid credentials).
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::{anyhow, Result};
    /// use axum_anyhow::{ApiResult, ResultExt};
    /// use axum::http::StatusCode;
    ///
    /// fn verify_token(token: Option<&str>) -> Result<String> {
    ///     token.ok_or_else(|| anyhow!("Missing token")).map(String::from)
    /// }
    ///
    /// let result = verify_token(None)
    ///     .context_unauthorized("Unauthorized", "Valid authentication token required");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::UNAUTHORIZED);
    /// ```
    fn context_unauthorized(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts an error to a 403 Forbidden error (authenticated but lacks permissions).
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::{anyhow, Result};
    /// use axum_anyhow::{ApiResult, ResultExt};
    /// use axum::http::StatusCode;
    ///
    /// fn check_admin(is_admin: bool) -> Result<()> {
    ///     if is_admin { Ok(()) } else { Err(anyhow!("Not admin")) }
    /// }
    ///
    /// let result = check_admin(false)
    ///     .context_forbidden("Forbidden", "Admin access required");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::FORBIDDEN);
    /// ```
    fn context_forbidden(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts an error to a 404 Not Found error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::{anyhow, Result};
    /// use axum_anyhow::{ApiResult, ResultExt};
    /// use axum::http::StatusCode;
    ///
    /// fn find_user(id: u32) -> Result<String> {
    ///     Err(anyhow!("User {} not found", id))
    /// }
    ///
    /// let result = find_user(123)
    ///     .context_not_found("Not Found", "The requested user does not exist");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::NOT_FOUND);
    /// ```
    fn context_not_found(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts an error to a 405 Method Not Allowed error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::{anyhow, Result};
    /// use axum_anyhow::{ApiResult, ResultExt};
    /// use axum::http::StatusCode;
    ///
    /// fn check_method(method: &str) -> Result<()> {
    ///     if method == "GET" { Ok(()) } else { Err(anyhow!("Invalid method")) }
    /// }
    ///
    /// let result = check_method("POST")
    ///     .context_method_not_allowed("Method Not Allowed", "Only GET requests are supported");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::METHOD_NOT_ALLOWED);
    /// ```
    fn context_method_not_allowed(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts an error to a 409 Conflict error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::{anyhow, Result};
    /// use axum_anyhow::{ApiResult, ResultExt};
    /// use axum::http::StatusCode;
    ///
    /// fn create_user(email: &str) -> Result<()> {
    ///     Err(anyhow!("Duplicate email"))
    /// }
    ///
    /// let result = create_user("test@example.com")
    ///     .context_conflict("Conflict", "A user with this email already exists");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::CONFLICT);
    /// ```
    fn context_conflict(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts an error to a 422 Unprocessable Entity error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::{anyhow, Result};
    /// use axum_anyhow::{ApiResult, ResultExt};
    /// use axum::http::StatusCode;
    ///
    /// fn validate_password(password: &str) -> Result<()> {
    ///     if password.len() >= 8 { Ok(()) } else { Err(anyhow!("Too short")) }
    /// }
    ///
    /// let result = validate_password("123")
    ///     .context_unprocessable_entity("Validation Failed", "Password must be at least 8 characters");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::UNPROCESSABLE_ENTITY);
    /// ```
    fn context_unprocessable_entity(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts an error to a 429 Too Many Requests error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::{anyhow, Result};
    /// use axum_anyhow::{ApiResult, ResultExt};
    /// use axum::http::StatusCode;
    ///
    /// fn check_rate_limit(count: u32) -> Result<()> {
    ///     if count < 100 { Ok(()) } else { Err(anyhow!("Rate limit exceeded")) }
    /// }
    ///
    /// let result = check_rate_limit(150)
    ///     .context_too_many_requests("Too Many Requests", "Rate limit exceeded. Please try again later");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::TOO_MANY_REQUESTS);
    /// ```
    fn context_too_many_requests(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts an error to a 500 Internal Server Error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::{anyhow, Result};
    /// use axum_anyhow::{ApiResult, ResultExt};
    /// use axum::http::StatusCode;
    ///
    /// fn connect_db() -> Result<()> {
    ///     Err(anyhow!("Connection failed"))
    /// }
    ///
    /// let result = connect_db()
    ///     .context_internal("Internal Error", "Database connection failed");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::INTERNAL_SERVER_ERROR);
    /// ```
    fn context_internal(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts an error to a 502 Bad Gateway error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::{anyhow, Result};
    /// use axum_anyhow::{ApiResult, ResultExt};
    /// use axum::http::StatusCode;
    ///
    /// fn call_upstream() -> Result<String> {
    ///     Err(anyhow!("Upstream error"))
    /// }
    ///
    /// let result = call_upstream()
    ///     .context_bad_gateway("Bad Gateway", "Upstream service returned an invalid response");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::BAD_GATEWAY);
    /// ```
    fn context_bad_gateway(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts an error to a 503 Service Unavailable error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::{anyhow, Result};
    /// use axum_anyhow::{ApiResult, ResultExt};
    /// use axum::http::StatusCode;
    ///
    /// fn check_service() -> Result<()> {
    ///     Err(anyhow!("Service down"))
    /// }
    ///
    /// let result = check_service()
    ///     .context_service_unavailable("Service Unavailable", "Service is currently under maintenance");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::SERVICE_UNAVAILABLE);
    /// ```
    fn context_service_unavailable(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts an error to a 504 Gateway Timeout error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use anyhow::{anyhow, Result};
    /// use axum_anyhow::{ApiResult, ResultExt};
    /// use axum::http::StatusCode;
    ///
    /// fn call_slow_service() -> Result<String> {
    ///     Err(anyhow!("Timeout"))
    /// }
    ///
    /// let result = call_slow_service()
    ///     .context_gateway_timeout("Gateway Timeout", "Upstream service did not respond in time");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::GATEWAY_TIMEOUT);
    /// ```
    fn context_gateway_timeout(self, title: &str, detail: &str) -> ApiResult<T>;
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

    fn context_method_not_allowed(self, title: &str, detail: &str) -> ApiResult<T> {
        self.map_err(|err| err.context_method_not_allowed(title, detail))
    }

    fn context_conflict(self, title: &str, detail: &str) -> ApiResult<T> {
        self.map_err(|err| err.context_conflict(title, detail))
    }

    fn context_unprocessable_entity(self, title: &str, detail: &str) -> ApiResult<T> {
        self.map_err(|err| err.context_unprocessable_entity(title, detail))
    }

    fn context_too_many_requests(self, title: &str, detail: &str) -> ApiResult<T> {
        self.map_err(|err| err.context_too_many_requests(title, detail))
    }

    fn context_internal(self, title: &str, detail: &str) -> ApiResult<T> {
        self.map_err(|err| err.context_internal(title, detail))
    }

    fn context_bad_gateway(self, title: &str, detail: &str) -> ApiResult<T> {
        self.map_err(|err| err.context_bad_gateway(title, detail))
    }

    fn context_service_unavailable(self, title: &str, detail: &str) -> ApiResult<T> {
        self.map_err(|err| err.context_service_unavailable(title, detail))
    }

    fn context_gateway_timeout(self, title: &str, detail: &str) -> ApiResult<T> {
        self.map_err(|err| err.context_gateway_timeout(title, detail))
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
///
/// # Sealed Trait
///
/// This trait is sealed and cannot be implemented for types outside this crate.
/// This is intentional to allow adding new methods in the future without breaking changes.
///
/// ```compile_fail
/// use axum_anyhow::OptionExt;
/// use axum::http::StatusCode;
///
/// struct MyOption<T>(Option<T>);
///
/// // This will not compile - trait is sealed
/// impl<T> OptionExt<T> for MyOption<T> {
///     fn context_status(self, status: StatusCode, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> {
///         todo!()
///     }
///     fn context_bad_request(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_unauthorized(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_forbidden(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_not_found(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_method_not_allowed(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_conflict(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_unprocessable_entity(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_too_many_requests(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_internal(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_bad_gateway(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_service_unavailable(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
///     fn context_gateway_timeout(self, title: &str, detail: &str) -> axum_anyhow::ApiResult<T> { todo!() }
/// }
/// ```
pub trait OptionExt<T>: sealed::SealedOption {
    /// Converts `None` to an `ApiError` with a custom status code.
    ///
    /// # Arguments
    ///
    /// * `status` - The HTTP status code to use
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum_anyhow::{ApiResult, OptionExt};
    /// use axum::http::StatusCode;
    ///
    /// fn get_coffee() -> Option<String> {
    ///     None  // No coffee available
    /// }
    ///
    /// let result: ApiResult<String> = get_coffee()
    ///     .context_status(StatusCode::IM_A_TEAPOT, "I'm a teapot", "Cannot brew coffee with a teapot");
    /// assert!(result.is_err());
    /// let err = result.unwrap_err();
    /// assert_eq!(err.status, StatusCode::IM_A_TEAPOT);
    /// assert_eq!(err.title, "I'm a teapot");
    /// assert_eq!(err.detail, "Cannot brew coffee with a teapot");
    /// ```
    fn context_status(self, status: StatusCode, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 400 Bad Request error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum_anyhow::{ApiResult, OptionExt};
    /// use axum::http::StatusCode;
    ///
    /// fn get_query_param(params: &[(&str, &str)], key: &str) -> Option<String> {
    ///     params.iter().find(|(k, _)| *k == key).map(|(_, v)| v.to_string())
    /// }
    ///
    /// let params = vec![("name", "Alice")];
    /// let result = get_query_param(&params, "age")
    ///     .context_bad_request("Missing Parameter", "Required parameter 'age' is missing");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::BAD_REQUEST);
    /// ```
    fn context_bad_request(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 401 Unauthorized error (missing or invalid credentials).
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum_anyhow::{ApiResult, OptionExt};
    /// use axum::http::StatusCode;
    ///
    /// fn get_auth_token(headers: &[(&str, &str)]) -> Option<String> {
    ///     headers.iter().find(|(k, _)| *k == "Authorization").map(|(_, v)| v.to_string())
    /// }
    ///
    /// let headers = vec![("Content-Type", "application/json")];
    /// let result = get_auth_token(&headers)
    ///     .context_unauthorized("Unauthorized", "Authentication token is required");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::UNAUTHORIZED);
    /// ```
    fn context_unauthorized(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 403 Forbidden error (authenticated but lacks permissions).
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum_anyhow::{ApiResult, OptionExt};
    /// use axum::http::StatusCode;
    ///
    /// fn get_admin_privilege(user_id: u32) -> Option<bool> {
    ///     if user_id == 1 { Some(true) } else { None }
    /// }
    ///
    /// let result = get_admin_privilege(42)
    ///     .context_forbidden("Forbidden", "Admin privileges required to access this resource");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::FORBIDDEN);
    /// ```
    fn context_forbidden(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 404 Not Found error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_not_found(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 405 Method Not Allowed error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum_anyhow::{ApiResult, OptionExt};
    /// use axum::http::StatusCode;
    ///
    /// fn get_allowed_method(method: &str) -> Option<String> {
    ///     if method == "GET" { Some(method.to_string()) } else { None }
    /// }
    ///
    /// let result = get_allowed_method("POST")
    ///     .context_method_not_allowed("Method Not Allowed", "This endpoint only supports GET requests");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::METHOD_NOT_ALLOWED);
    /// ```
    fn context_method_not_allowed(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 409 Conflict error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum_anyhow::{ApiResult, OptionExt};
    /// use axum::http::StatusCode;
    ///
    /// fn try_reserve_username(username: &str) -> Option<String> {
    ///     // Returns None if username already taken
    ///     None
    /// }
    ///
    /// let result = try_reserve_username("admin")
    ///     .context_conflict("Conflict", "Username is already taken");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::CONFLICT);
    /// ```
    fn context_conflict(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 422 Unprocessable Entity error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum_anyhow::{ApiResult, OptionExt};
    /// use axum::http::StatusCode;
    ///
    /// fn validate_email_format(email: &str) -> Option<String> {
    ///     if email.contains('@') { Some(email.to_string()) } else { None }
    /// }
    ///
    /// let result = validate_email_format("invalid-email")
    ///     .context_unprocessable_entity("Validation Failed", "Email must contain an @ symbol");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::UNPROCESSABLE_ENTITY);
    /// ```
    fn context_unprocessable_entity(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 429 Too Many Requests error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum_anyhow::{ApiResult, OptionExt};
    /// use axum::http::StatusCode;
    ///
    /// fn check_rate_limit_quota(user_id: u32) -> Option<u32> {
    ///     // Returns remaining quota or None if exceeded
    ///     None
    /// }
    ///
    /// let result = check_rate_limit_quota(123)
    ///     .context_too_many_requests("Too Many Requests", "API rate limit exceeded. Please try again later");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::TOO_MANY_REQUESTS);
    /// ```
    fn context_too_many_requests(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 500 Internal Server Error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum_anyhow::{ApiResult, OptionExt};
    /// use axum::http::StatusCode;
    ///
    /// fn get_config_value(key: &str) -> Option<String> {
    ///     // Returns None if critical config is missing
    ///     None
    /// }
    ///
    /// let result = get_config_value("database_url")
    ///     .context_internal("Internal Error", "Critical configuration missing");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::INTERNAL_SERVER_ERROR);
    /// ```
    fn context_internal(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 502 Bad Gateway error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum_anyhow::{ApiResult, OptionExt};
    /// use axum::http::StatusCode;
    ///
    /// fn parse_upstream_response(response: &str) -> Option<String> {
    ///     // Returns None if response is invalid
    ///     None
    /// }
    ///
    /// let result = parse_upstream_response("invalid")
    ///     .context_bad_gateway("Bad Gateway", "Upstream service returned invalid response");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::BAD_GATEWAY);
    /// ```
    fn context_bad_gateway(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 503 Service Unavailable error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum_anyhow::{ApiResult, OptionExt};
    /// use axum::http::StatusCode;
    ///
    /// fn get_service_status() -> Option<bool> {
    ///     // Returns None if service is unavailable
    ///     None
    /// }
    ///
    /// let result = get_service_status()
    ///     .context_service_unavailable("Service Unavailable", "Service is temporarily down for maintenance");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::SERVICE_UNAVAILABLE);
    /// ```
    fn context_service_unavailable(self, title: &str, detail: &str) -> ApiResult<T>;

    /// Converts `None` to a 504 Gateway Timeout error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum_anyhow::{ApiResult, OptionExt};
    /// use axum::http::StatusCode;
    ///
    /// fn wait_for_upstream_response(timeout_ms: u64) -> Option<String> {
    ///     // Returns None if timeout exceeded
    ///     None
    /// }
    ///
    /// let result = wait_for_upstream_response(5000)
    ///     .context_gateway_timeout("Gateway Timeout", "Upstream service did not respond within timeout");
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().status, StatusCode::GATEWAY_TIMEOUT);
    /// ```
    fn context_gateway_timeout(self, title: &str, detail: &str) -> ApiResult<T>;
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

    fn context_method_not_allowed(self, title: &str, detail: &str) -> ApiResult<T> {
        self.context_status(StatusCode::METHOD_NOT_ALLOWED, title, detail)
    }

    fn context_conflict(self, title: &str, detail: &str) -> ApiResult<T> {
        self.context_status(StatusCode::CONFLICT, title, detail)
    }

    fn context_unprocessable_entity(self, title: &str, detail: &str) -> ApiResult<T> {
        self.context_status(StatusCode::UNPROCESSABLE_ENTITY, title, detail)
    }

    fn context_too_many_requests(self, title: &str, detail: &str) -> ApiResult<T> {
        self.context_status(StatusCode::TOO_MANY_REQUESTS, title, detail)
    }

    fn context_internal(self, title: &str, detail: &str) -> ApiResult<T> {
        self.context_status(StatusCode::INTERNAL_SERVER_ERROR, title, detail)
    }

    fn context_bad_gateway(self, title: &str, detail: &str) -> ApiResult<T> {
        self.context_status(StatusCode::BAD_GATEWAY, title, detail)
    }

    fn context_service_unavailable(self, title: &str, detail: &str) -> ApiResult<T> {
        self.context_status(StatusCode::SERVICE_UNAVAILABLE, title, detail)
    }

    fn context_gateway_timeout(self, title: &str, detail: &str) -> ApiResult<T> {
        self.context_status(StatusCode::GATEWAY_TIMEOUT, title, detail)
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
///
/// # Sealed Trait
///
/// This trait is sealed and cannot be implemented for types outside this crate.
/// This is intentional to allow adding new methods in the future without breaking changes.
///
/// ```compile_fail
/// use axum_anyhow::IntoApiError;
/// use axum::http::StatusCode;
///
/// struct MyError;
///
/// // This will not compile - trait is sealed
/// impl IntoApiError for MyError {
///     fn context_status(self, status: StatusCode, title: &str, detail: &str) -> axum_anyhow::ApiError {
///         todo!()
///     }
///     fn context_bad_request(self, title: &str, detail: &str) -> axum_anyhow::ApiError { todo!() }
///     fn context_unauthorized(self, title: &str, detail: &str) -> axum_anyhow::ApiError { todo!() }
///     fn context_forbidden(self, title: &str, detail: &str) -> axum_anyhow::ApiError { todo!() }
///     fn context_not_found(self, title: &str, detail: &str) -> axum_anyhow::ApiError { todo!() }
///     fn context_method_not_allowed(self, title: &str, detail: &str) -> axum_anyhow::ApiError { todo!() }
///     fn context_conflict(self, title: &str, detail: &str) -> axum_anyhow::ApiError { todo!() }
///     fn context_unprocessable_entity(self, title: &str, detail: &str) -> axum_anyhow::ApiError { todo!() }
///     fn context_too_many_requests(self, title: &str, detail: &str) -> axum_anyhow::ApiError { todo!() }
///     fn context_internal(self, title: &str, detail: &str) -> axum_anyhow::ApiError { todo!() }
///     fn context_bad_gateway(self, title: &str, detail: &str) -> axum_anyhow::ApiError { todo!() }
///     fn context_service_unavailable(self, title: &str, detail: &str) -> axum_anyhow::ApiError { todo!() }
///     fn context_gateway_timeout(self, title: &str, detail: &str) -> axum_anyhow::ApiError { todo!() }
/// }
/// ```
pub trait IntoApiError: sealed::SealedIntoApiError {
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

    /// Converts an error to a 405 Method Not Allowed error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_method_not_allowed(self, title: &str, detail: &str) -> ApiError;

    /// Converts an error to a 409 Conflict error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_conflict(self, title: &str, detail: &str) -> ApiError;

    /// Converts an error to a 422 Unprocessable Entity error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_unprocessable_entity(self, title: &str, detail: &str) -> ApiError;

    /// Converts an error to a 429 Too Many Requests error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_too_many_requests(self, title: &str, detail: &str) -> ApiError;

    /// Converts an error to a 500 Internal Server Error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_internal(self, title: &str, detail: &str) -> ApiError;

    /// Converts an error to a 502 Bad Gateway error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_bad_gateway(self, title: &str, detail: &str) -> ApiError;

    /// Converts an error to a 503 Service Unavailable error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_service_unavailable(self, title: &str, detail: &str) -> ApiError;

    /// Converts an error to a 504 Gateway Timeout error.
    ///
    /// # Arguments
    ///
    /// * `title` - A short, human-readable summary of the error
    /// * `detail` - A detailed explanation of the error
    fn context_gateway_timeout(self, title: &str, detail: &str) -> ApiError;
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

    fn context_method_not_allowed(self, title: &str, detail: &str) -> ApiError {
        self.context_status(StatusCode::METHOD_NOT_ALLOWED, title, detail)
    }

    fn context_conflict(self, title: &str, detail: &str) -> ApiError {
        self.context_status(StatusCode::CONFLICT, title, detail)
    }

    fn context_unprocessable_entity(self, title: &str, detail: &str) -> ApiError {
        self.context_status(StatusCode::UNPROCESSABLE_ENTITY, title, detail)
    }

    fn context_too_many_requests(self, title: &str, detail: &str) -> ApiError {
        self.context_status(StatusCode::TOO_MANY_REQUESTS, title, detail)
    }

    fn context_internal(self, title: &str, detail: &str) -> ApiError {
        self.context_status(StatusCode::INTERNAL_SERVER_ERROR, title, detail)
    }

    fn context_bad_gateway(self, title: &str, detail: &str) -> ApiError {
        self.context_status(StatusCode::BAD_GATEWAY, title, detail)
    }

    fn context_service_unavailable(self, title: &str, detail: &str) -> ApiError {
        self.context_status(StatusCode::SERVICE_UNAVAILABLE, title, detail)
    }

    fn context_gateway_timeout(self, title: &str, detail: &str) -> ApiError {
        self.context_status(StatusCode::GATEWAY_TIMEOUT, title, detail)
    }
}

mod sealed {
    use crate::IntoApiError;

    pub trait SealedResult {}
    pub trait SealedOption {}
    pub trait SealedIntoApiError {}

    impl<T, E> SealedResult for Result<T, E> where E: IntoApiError {}
    impl<T> SealedOption for Option<T> {}
    impl<E> SealedIntoApiError for E where E: Into<anyhow::Error> {}
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
    fn test_into_api_error_context_status() {
        let anyhow_err = anyhow!("Custom error");
        let api_err = anyhow_err.context_status(StatusCode::IM_A_TEAPOT, "Teapot", "I'm a teapot");

        assert_eq!(api_err.status, StatusCode::IM_A_TEAPOT);
        assert_eq!(api_err.title, "Teapot");
        assert_eq!(api_err.detail, "I'm a teapot");
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
