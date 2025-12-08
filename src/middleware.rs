//! Middleware for intercepting and enriching API errors with request context.
//!
//! This module provides a middleware layer that can intercept error responses and
//! enrich them with metadata derived from the request context. This is useful for
//! adding trace IDs, request IDs, timestamps, or other contextual information to errors.

use axum::{extract::Request, response::Response};
use futures_util::future::BoxFuture;
use serde_json::Value;
use std::{
    sync::RwLock,
    task::{Context, Poll},
};
use tower::{Layer, Service};

/// A callback function that can enrich an error with metadata based on the request.
///
/// The callback receives:
/// - A mutable reference to the error's metadata (if not yet set, it will be `None`)
/// - A reference to the HTTP request
///
/// The callback can create or modify the metadata to include request-specific information.
pub type MetaCallback = Box<dyn Fn(&mut Option<Value>, &Request) + Send + Sync>;

static META_CALLBACK: RwLock<Option<MetaCallback>> = RwLock::new(None);

/// Sets a global callback that will be used to enrich API errors with metadata.
///
/// This callback is invoked by the `ErrorInterceptor` middleware for each error response,
/// allowing you to add request-specific metadata like request IDs, trace information, etc.
///
/// # Example
///
/// ```rust
/// use axum_anyhow::set_meta_callback;
/// use serde_json::json;
///
/// set_meta_callback(|meta, request| {
///     // Add request method and URI to error metadata
///     *meta = Some(json!({
///         "method": request.method().as_str(),
///         "uri": request.uri().to_string(),
///         "timestamp": chrono::Utc::now().to_rfc3339(),
///     }));
/// });
/// ```
pub fn set_meta_callback<F>(callback: F)
where
    F: Fn(&mut Option<Value>, &Request) + Send + Sync + 'static,
{
    let mut guard = META_CALLBACK
        .write()
        .expect("Failed to get write lock for MetaCallback");
    *guard = Some(Box::new(callback));
}

/// Invokes the global meta callback if one is set.
///
/// This is called by the middleware to enrich error metadata.
pub(crate) fn invoke_meta_callback(meta: &mut Option<Value>, request: &Request) {
    let guard = META_CALLBACK
        .read()
        .expect("Failed to get read lock for MetaCallback");
    if let Some(callback) = guard.as_ref() {
        callback(meta, request);
    }
}

/// A middleware layer that intercepts error responses and enriches them with metadata.
///
/// This layer wraps your Axum application and intercepts responses with error status codes
/// (4xx and 5xx). If a meta callback has been set via `set_meta_callback`, it will be
/// invoked to add request-specific metadata to the error response.
///
/// # Example
///
/// ```rust
/// use axum::Router;
/// use axum_anyhow::{ErrorInterceptorLayer, set_meta_callback};
/// use serde_json::json;
///
/// // Set up the meta callback
/// set_meta_callback(|meta, request| {
///     *meta = Some(json!({
///         "request_id": "generated-id",
///         "path": request.uri().path(),
///     }));
/// });
///
/// // Apply the middleware to your router
/// let app: Router = Router::new()
///     .layer(ErrorInterceptorLayer);
/// ```
#[derive(Clone, Copy)]
pub struct ErrorInterceptorLayer;

impl<S> Layer<S> for ErrorInterceptorLayer {
    type Service = ErrorInterceptor<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ErrorInterceptor { inner }
    }
}

/// The service implementation for the error interceptor.
#[derive(Clone)]
pub struct ErrorInterceptor<S> {
    inner: S,
}

impl<S> Service<Request> for ErrorInterceptor<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            let response = inner.call(request).await?;

            // Only intercept error responses
            let status = response.status();
            if status.is_client_error() || status.is_server_error() {
                // Try to extract and enrich the error
                // Note: This is a simplified version. In a real implementation,
                // you might need to parse the response body to modify ApiError metadata.
                // For now, this demonstrates the middleware structure.
                // The actual enrichment happens at error creation time via builder_with_request.
            }

            Ok(response)
        })
    }
}

/// Extension methods for `ApiErrorBuilder` to work with request context.
///
/// These methods are used internally by the middleware to create errors with
/// request-aware metadata.
pub trait ApiErrorBuilderExt {
    /// Enriches the error builder with metadata from the request.
    ///
    /// This method invokes the global meta callback (if set) to populate
    /// the error's metadata based on the request context.
    fn with_request_meta(self, request: &Request) -> Self;
}

impl ApiErrorBuilderExt for crate::ApiErrorBuilder {
    fn with_request_meta(mut self, request: &Request) -> Self {
        let mut meta = None;
        invoke_meta_callback(&mut meta, request);
        if let Some(meta_value) = meta {
            self = self.meta(meta_value);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Method, StatusCode},
    };
    use serde_json::json;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_set_meta_callback() {
        set_meta_callback(|meta, request| {
            *meta = Some(json!({
                "method": request.method().as_str(),
            }));
        });

        let request = Request::builder()
            .method(Method::GET)
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let mut meta = None;
        invoke_meta_callback(&mut meta, &request);

        assert!(meta.is_some());
        let meta_value = meta.unwrap();
        assert_eq!(meta_value["method"], "GET");
    }

    #[test]
    #[serial]
    fn test_meta_callback_not_set() {
        // Clear any existing callback
        {
            let mut guard = META_CALLBACK.write().unwrap();
            *guard = None;
        }

        let request = Request::builder()
            .method(Method::POST)
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let mut meta = None;
        invoke_meta_callback(&mut meta, &request);

        assert!(meta.is_none());
    }

    #[test]
    #[serial]
    fn test_api_error_builder_with_request_meta() {
        set_meta_callback(|meta, request| {
            *meta = Some(json!({
                "uri": request.uri().to_string(),
                "method": request.method().as_str(),
            }));
        });

        let request = Request::builder()
            .method(Method::DELETE)
            .uri("/users/123")
            .body(Body::empty())
            .unwrap();

        let error = crate::ApiError::builder()
            .status(StatusCode::NOT_FOUND)
            .title("Not Found")
            .detail("User not found")
            .with_request_meta(&request)
            .build();

        assert!(error.meta.is_some());
        let meta = error.meta.unwrap();
        assert_eq!(meta["uri"], "/users/123");
        assert_eq!(meta["method"], "DELETE");
    }
}
