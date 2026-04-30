/// Context for an API error: a title and an optional detail.
///
/// Methods on [`ResultExt`](crate::ResultExt), [`OptionExt`](crate::OptionExt), and
/// [`IntoApiError`](crate::IntoApiError) accept `impl Into<Context>`, so you can pass
/// either a bare title or a `(title, detail)` pair.
///
/// # Example
///
/// ```rust
/// use axum_anyhow::{ApiResult, ResultExt};
/// use anyhow::anyhow;
///
/// fn fallible() -> anyhow::Result<()> {
///     Err(anyhow!("oops"))
/// }
///
/// // title only — detail is None
/// let _: ApiResult<()> = fallible().context_not_found("Not Found");
///
/// // title + detail
/// let _: ApiResult<()> = fallible().context_not_found(("Not Found", "The resource does not exist"));
/// ```
pub struct ApiErrorContext {
    pub(crate) title: String,
    pub(crate) detail: Option<String>,
}

impl From<&str> for ApiErrorContext {
    fn from(title: &str) -> Self {
        Self {
            title: title.to_string(),
            detail: None,
        }
    }
}

impl From<String> for ApiErrorContext {
    fn from(title: String) -> Self {
        Self {
            title,
            detail: None,
        }
    }
}

impl From<(&str, &str)> for ApiErrorContext {
    fn from((title, detail): (&str, &str)) -> Self {
        Self {
            title: title.to_string(),
            detail: Some(detail.to_string()),
        }
    }
}

impl From<(String, String)> for ApiErrorContext {
    fn from((title, detail): (String, String)) -> Self {
        Self {
            title,
            detail: Some(detail),
        }
    }
}

impl From<(&str, String)> for ApiErrorContext {
    fn from((title, detail): (&str, String)) -> Self {
        Self {
            title: title.to_string(),
            detail: Some(detail),
        }
    }
}

impl From<(String, &str)> for ApiErrorContext {
    fn from((title, detail): (String, &str)) -> Self {
        Self {
            title,
            detail: Some(detail.to_string()),
        }
    }
}
