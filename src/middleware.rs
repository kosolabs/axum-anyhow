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
use std::{
    cell::RefCell,
    sync::Arc,
    task::{Context, Poll},
};
use tower::{Layer, Service};

thread_local! {
    static ENRICHMENT_CONTEXT: RefCell<Option<EnrichmentContext>> = const { RefCell::new(None) };
}

/// Request information snapshot available to the error enricher.
///
/// This struct contains request metadata that can be used to enrich errors.
#[derive(Clone, Debug)]
pub struct RequestSnapshot {
    /// The HTTP method of the request
    method: Method,
    /// The URI of the request
    uri: Uri,
    /// The HTTP headers of the request
    headers: HeaderMap,
}

impl RequestSnapshot {
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

    /// Creates a `RequestSnapshot` from an Axum `Request`.
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

/// Type alias for the error enricher function.
type ErrorEnricher =
    Arc<dyn Fn(ApiErrorBuilder, &RequestSnapshot) -> ApiErrorBuilder + Send + Sync + 'static>;

/// Context for enriching errors with request information.
///
/// This struct combines the request context with the enricher callback,
/// making it easier to pass both pieces of data together through the middleware.
#[derive(Clone)]
pub(crate) struct EnrichmentContext {
    request: RequestSnapshot,
    enricher: ErrorEnricher,
}

impl EnrichmentContext {
    /// Creates a new `EnrichmentContext` with the given context and enricher.
    fn new(request: RequestSnapshot, enricher: ErrorEnricher) -> Self {
        Self { request, enricher }
    }

    /// Installs this enrichment context as the current thread-local data.
    fn set(self) {
        ENRICHMENT_CONTEXT.with(|data| {
            *data.borrow_mut() = Some(self);
        });
    }

    /// Removes the current thread-local enrichment context.
    fn clear() {
        ENRICHMENT_CONTEXT.with(|data| {
            *data.borrow_mut() = None;
        });
    }

    /// Applies the enricher to the given builder.
    fn apply(&self, builder: ApiErrorBuilder) -> ApiErrorBuilder {
        (self.enricher)(builder, &self.request)
    }

    /// Invokes the error enricher if one is set and request context is available.
    ///
    /// This is called internally by `ApiErrorBuilder::build()`.
    pub(crate) fn invoke(builder: ApiErrorBuilder) -> ApiErrorBuilder {
        ENRICHMENT_CONTEXT.with(|data| {
            if let Some(enrichment_ctx) = data.borrow().as_ref() {
                enrichment_ctx.apply(builder)
            } else {
                builder
            }
        })
    }
}

/// Service that captures request context and makes it available for error enrichment.
pub struct ErrorInterceptor<S> {
    inner: S,
    enricher: ErrorEnricher,
}

impl<S> Clone for ErrorInterceptor<S>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            enricher: self.enricher.clone(),
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
        let snapshot = RequestSnapshot::from_request(&request);
        let ctx = EnrichmentContext::new(snapshot, self.enricher.clone());

        let future = self.inner.call(request);

        Box::pin(async move {
            // Install enrichment context for this task
            ctx.set();

            // Call the inner service
            let result = future.await;

            // Remove enrichment context after request completes
            EnrichmentContext::clear();

            result
        })
    }
}

/// Middleware layer that enables error enrichment with request context.
///
/// This layer captures request information (method, URI, headers) and makes it available
/// to the error enricher callback.
///
/// # Example
///
/// ```rust
/// use axum::Router;
/// use axum_anyhow::ErrorInterceptorLayer;
/// use serde_json::json;
///
/// // Create the layer with an enricher
/// let enricher_layer = ErrorInterceptorLayer::new(|builder, ctx| {
///     builder.meta(json!({
///         "method": ctx.method().as_str(),
///         "uri": ctx.uri().to_string(),
///         "user_agent": ctx.headers()
///             .get("user-agent")
///             .and_then(|v| v.to_str().ok())
///             .unwrap_or("unknown"),
///     }))
/// });
///
/// // Apply the middleware
/// let app: Router = Router::new()
///     .layer(enricher_layer);
/// ```
#[derive(Clone)]
pub struct ErrorInterceptorLayer {
    enricher: ErrorEnricher,
}

impl ErrorInterceptorLayer {
    /// Creates a new `ErrorInterceptorLayer` with the given enricher function.
    ///
    /// The enricher will be called for every error created during request handling,
    /// allowing you to add request-specific metadata.
    pub fn new<F>(enricher: F) -> Self
    where
        F: Fn(ApiErrorBuilder, &RequestSnapshot) -> ApiErrorBuilder + Send + Sync + 'static,
    {
        Self {
            enricher: Arc::new(enricher),
        }
    }
}

impl<S> Layer<S> for ErrorInterceptorLayer {
    type Service = ErrorInterceptor<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ErrorInterceptor {
            inner,
            enricher: self.enricher.clone(),
        }
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
    fn test_error_enricher() {
        let enricher = Arc::new(|builder: ApiErrorBuilder, req: &RequestSnapshot| {
            builder.meta(json!({
                "method": req.method.as_str(),
                "uri": req.uri.to_string(),
            }))
        });

        // Set up request context with enricher
        let snapshot = RequestSnapshot {
            method: Method::GET,
            uri: "/test".parse().unwrap(),
            headers: HeaderMap::default(),
        };
        EnrichmentContext::new(snapshot, enricher).set();

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

        EnrichmentContext::clear();
    }

    #[test]
    #[serial]
    fn test_enricher_without_context() {
        // No request context set
        EnrichmentContext::clear();

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
    fn test_request_data_lifecycle() {
        let snapshot = RequestSnapshot {
            method: Method::POST,
            uri: "/api/users".parse().unwrap(),
            headers: HeaderMap::default(),
        };
        let enricher = Arc::new(|builder: ApiErrorBuilder, _req: &RequestSnapshot| builder);

        // Install enrichment context
        EnrichmentContext::new(snapshot.clone(), enricher).set();

        // Verify it's set
        ENRICHMENT_CONTEXT.with(|data| {
            let borrowed = data.borrow();
            assert!(borrowed.is_some());
            let stored_req = &borrowed.as_ref().unwrap().request;
            assert_eq!(stored_req.method, Method::POST);
            assert_eq!(stored_req.uri.to_string(), "/api/users");
        });

        // Remove enrichment context
        EnrichmentContext::clear();

        // Verify it's cleared
        ENRICHMENT_CONTEXT.with(|data| {
            assert!(data.borrow().is_none());
        });
    }
}
