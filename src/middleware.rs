//! Middleware for enriching API errors with request context.
//!
//! This module provides a middleware layer and global hook system for automatically
//! enriching errors with request-specific metadata like URIs, methods, headers, etc.

use crate::ApiErrorBuilder;
use axum::{
    extract::Request,
    http::{HeaderMap, Method, Uri},
    response::Response,
};
use futures_util::future::BoxFuture;
use std::sync::RwLock;
use std::task::{Context, Poll};
use tower::{Layer, Service};

/// Request context information available to the error enricher.
///
/// This struct contains request metadata that can be used to enrich errors.
#[derive(Clone, Debug)]
pub struct RequestContext {
    /// The HTTP method of the request
    method: Method,
    /// The URI of the request
    uri: Uri,
    /// The HTTP headers of the request
    headers: HeaderMap,
}

impl RequestContext {
    /// Returns a reference to the HTTP method of the request.
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Returns a reference to the URI of the request.
    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    /// Returns a reference to the HTTP headers of the request.
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Creates a `RequestContext` from an Axum `Request`.
    ///
    /// Extracts the method, URI, headers, and extensions from the request.
    pub fn from_request(request: &Request) -> Self {
        Self {
            method: request.method().clone(),
            uri: request.uri().clone(),
            headers: request.headers().clone(),
        }
    }
}

thread_local! {
    static REQUEST_CONTEXT: std::cell::RefCell<Option<RequestContext>> = const { std::cell::RefCell::new(None) };
}

/// Type alias for the error enricher function.
pub type ErrorEnricher = Box<dyn Fn(&mut ApiErrorBuilder, &RequestContext) + Send + Sync>;

static ERROR_ENRICHER: RwLock<Option<ErrorEnricher>> = RwLock::new(None);

/// Sets a global enricher that will be called when errors are built.
///
/// The enricher receives a mutable reference to the error builder and the request context,
/// allowing it to add metadata or modify other error fields based on the request.
///
/// # Example
///
/// ```rust
/// use axum_anyhow::set_error_enricher;
/// use serde_json::json;
///
/// set_error_enricher(|builder, ctx| {
///     *builder = builder.clone().meta(json!({
///         "method": ctx.method().as_str(),
///         "uri": ctx.uri().to_string(),
///         "user_agent": ctx.headers()
///             .get("user-agent")
///             .and_then(|v| v.to_str().ok())
///             .unwrap_or("unknown"),
///     }));
/// });
/// ```
pub fn set_error_enricher<F>(enricher: F)
where
    F: Fn(&mut ApiErrorBuilder, &RequestContext) + Send + Sync + 'static,
{
    let mut guard = ERROR_ENRICHER
        .write()
        .expect("Failed to get write lock for ErrorEnricher");
    *guard = Some(Box::new(enricher));
}

/// Invokes the global error enricher if one is set and request context is available.
///
/// This is called internally by `ApiErrorBuilder::build()`.
pub(crate) fn invoke_enricher(builder: &mut ApiErrorBuilder) {
    REQUEST_CONTEXT.with(|ctx| {
        if let Some(request_ctx) = ctx.borrow().as_ref() {
            let guard = ERROR_ENRICHER
                .read()
                .expect("Failed to get read lock for ErrorEnricher");
            if let Some(enricher) = guard.as_ref() {
                enricher(builder, request_ctx);
            }
        }
    });
}

/// Sets the request context for the current task.
///
/// This is called by the middleware to make request information available
/// to error enrichment.
fn set_request_context(ctx: RequestContext) {
    REQUEST_CONTEXT.with(|c| {
        *c.borrow_mut() = Some(ctx);
    });
}

/// Clears the request context for the current task.
fn clear_request_context() {
    REQUEST_CONTEXT.with(|c| {
        *c.borrow_mut() = None;
    });
}

/// Service that captures request context and makes it available for error enrichment.
pub struct ErrorInterceptor<S> {
    inner: S,
}

impl<S> Clone for ErrorInterceptor<S>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<S> Service<Request> for ErrorInterceptor<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        // Capture request context
        let ctx = RequestContext::from_request(&request);

        let future = self.inner.call(request);

        Box::pin(async move {
            // Set context for this task
            set_request_context(ctx);

            // Call the inner service
            let result = future.await;

            // Clear context after request completes
            clear_request_context();

            result
        })
    }
}

/// Middleware layer that enables error enrichment with request context.
///
/// This layer captures request information (method, URI) and makes it available
/// to the error enricher callback set via [`set_error_enricher`].
///
/// # Example
///
/// ```rust
/// use axum::Router;
/// use axum_anyhow::{ErrorInterceptorLayer, set_error_enricher};
/// use serde_json::json;
///
/// // Set up the enricher
/// set_error_enricher(|builder, ctx| {
///     *builder = builder.clone().meta(json!({
///         "method": ctx.method().as_str(),
///         "uri": ctx.uri().to_string(),
///         "user_agent": ctx.headers()
///             .get("user-agent")
///             .and_then(|v| v.to_str().ok())
///             .unwrap_or("unknown"),
///     }));
/// });
///
/// // Apply the middleware
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use serde_json::json;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_set_error_enricher() {
        set_error_enricher(|builder, ctx| {
            *builder = builder.clone().meta(json!({
                "method": ctx.method.as_str(),
                "uri": ctx.uri.to_string(),
            }));
        });

        // Set up request context
        let ctx = RequestContext {
            method: Method::GET,
            uri: "/test".parse().unwrap(),
            headers: HeaderMap::default(),
        };
        set_request_context(ctx);

        // Build an error
        let error = crate::ApiError::builder()
            .status(StatusCode::NOT_FOUND)
            .title("Not Found")
            .detail("Resource not found")
            .build();

        // Verify enrichment happened
        assert!(error.meta().is_some());
        let meta = error.meta().unwrap();
        assert_eq!(meta["method"], "GET");
        assert_eq!(meta["uri"], "/test");

        clear_request_context();
    }

    #[test]
    #[serial]
    fn test_enricher_without_context() {
        set_error_enricher(|builder, ctx| {
            *builder = builder.clone().meta(json!({
                "method": ctx.method.as_str(),
            }));
        });

        // No request context set
        clear_request_context();

        // Build an error
        let error = crate::ApiError::builder()
            .status(StatusCode::BAD_REQUEST)
            .title("Bad Request")
            .detail("Invalid input")
            .build();

        // Enrichment should not happen without context
        assert!(error.meta().is_none());
    }

    #[test]
    #[serial]
    fn test_request_context_lifecycle() {
        let ctx = RequestContext {
            method: Method::POST,
            uri: "/api/users".parse().unwrap(),
            headers: HeaderMap::default(),
        };

        // Set context
        set_request_context(ctx.clone());

        // Verify it's set
        REQUEST_CONTEXT.with(|c| {
            let borrowed = c.borrow();
            assert!(borrowed.is_some());
            let stored = borrowed.as_ref().unwrap();
            assert_eq!(stored.method, Method::POST);
            assert_eq!(stored.uri.to_string(), "/api/users");
        });

        // Clear context
        clear_request_context();

        // Verify it's cleared
        REQUEST_CONTEXT.with(|c| {
            assert!(c.borrow().is_none());
        });
    }
}
