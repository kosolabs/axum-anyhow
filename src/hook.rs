use crate::ApiError;
use std::sync::RwLock;

type ErrorHook = Box<dyn Fn(&ApiError) + Send + Sync>;
static ERROR_HOOK: RwLock<Option<ErrorHook>> = RwLock::new(None);

/// Sets a global hook that will be called whenever an ApiError is created.
///
/// # Example
/// ```
/// use tracing;
/// use axum::http::StatusCode;
/// use axum_anyhow::ApiError;
///
/// axum_anyhow::on_error(|err| {
///     tracing::error!("Failed: {} ({}): {}", err.status, err.title, err.detail)
/// });
///
/// // The hook set above will get called once we build an ApiError.
/// ApiError::builder()
///     .status(StatusCode::BAD_REQUEST)
///     .title("Test Error")
///     .detail("This is a test")
///     .build();
/// ```
pub fn on_error<F>(hook: F)
where
    F: Fn(&ApiError) + Send + Sync + 'static,
{
    let mut guard = ERROR_HOOK
        .write()
        .expect("Failed to get write lock for ErrorHook");
    *guard = Some(Box::new(hook));
}

pub(crate) fn invoke_hook(error: &ApiError) {
    let guard = ERROR_HOOK
        .read()
        .expect("Failed get read lock for ErrorHook");
    if let Some(hook) = guard.as_ref() {
        hook(error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use serial_test::serial;
    use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
    use std::sync::{Arc, Mutex};

    #[test]
    #[serial]
    fn test_hook_is_called_when_error_is_built() {
        // Track if the hook was called
        let called = Arc::new(AtomicBool::new(false));

        // Set up the hook
        on_error({
            let called = called.clone();
            move |_err| {
                called.store(true, Ordering::SeqCst);
            }
        });

        // Create an error which should trigger the hook
        let _error = ApiError::builder()
            .status(StatusCode::BAD_REQUEST)
            .title("Test Error")
            .detail("This is a test")
            .build();

        // Verify the hook was called
        assert!(
            called.load(Ordering::SeqCst),
            "Hook should have been called"
        );
    }

    #[test]
    #[serial]
    fn test_hook_receives_correct_error_details() {
        // Track the error details passed to the hook
        let captured_status = Arc::new(Mutex::new(None));
        let captured_title = Arc::new(Mutex::new(None));
        let captured_detail = Arc::new(Mutex::new(None));

        on_error({
            let captured_status = captured_status.clone();
            let captured_title = captured_title.clone();
            let captured_detail = captured_detail.clone();

            move |err| {
                *captured_status.lock().unwrap() = Some(err.status);
                *captured_title.lock().unwrap() = Some(err.title.clone());
                *captured_detail.lock().unwrap() = Some(err.detail.clone());
            }
        });

        // Create an error with specific details
        let _error = ApiError::builder()
            .status(StatusCode::NOT_FOUND)
            .title("Resource Not Found")
            .detail("The requested resource does not exist")
            .build();

        // Verify the hook received the correct details
        assert_eq!(
            *captured_status.lock().unwrap(),
            Some(StatusCode::NOT_FOUND)
        );
        assert_eq!(
            *captured_title.lock().unwrap(),
            Some("Resource Not Found".to_string())
        );
        assert_eq!(
            *captured_detail.lock().unwrap(),
            Some("The requested resource does not exist".to_string())
        );
    }

    #[test]
    #[serial]
    fn test_hook_can_be_replaced() {
        let first_call = Arc::new(AtomicU8::new(0));
        let second_call = Arc::new(AtomicU8::new(0));

        // Set first hook
        on_error({
            let first_call = first_call.clone();
            move |_err| {
                first_call.fetch_add(1, Ordering::SeqCst);
            }
        });

        // Create an error - should call first hook
        let _error1 = ApiError::builder()
            .status(StatusCode::BAD_REQUEST)
            .title("Error 1")
            .build();

        // Replace with second hook
        on_error({
            let second_call = second_call.clone();
            move |_err| {
                second_call.fetch_add(1, Ordering::SeqCst);
            }
        });

        // Create another error - should call second hook only
        let _error2 = ApiError::builder()
            .status(StatusCode::BAD_REQUEST)
            .title("Error 2")
            .build();

        // First hook should have been called once, second hook should have been called once
        assert_eq!(first_call.load(Ordering::SeqCst), 1);
        assert_eq!(second_call.load(Ordering::SeqCst), 1);
    }

    #[test]
    #[serial]
    fn test_invoke_hook_without_setting_hook() {
        // This should not panic - it should just do nothing
        let error = ApiError::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .title("Test")
            .build();

        // If we get here without panicking, the test passes
        assert_eq!(error.status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    #[serial]
    fn test_hook_with_multiple_errors() {
        let counter = Arc::new(AtomicU8::new(0));

        on_error({
            let counter = counter.clone();
            move |_err| {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });

        // Create multiple errors
        for i in 0..5 {
            let _error = ApiError::builder()
                .status(StatusCode::BAD_REQUEST)
                .title(format!("Error {}", i))
                .build();
        }

        // Hook should have been called 5 times
        assert_eq!(counter.load(Ordering::SeqCst), 5);
    }
}
