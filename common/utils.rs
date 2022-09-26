use std::error::Error;

use crate::internal::shared_reconciler_rust_libraries::models::entities::app_errors::{AppError, AppErrorKind};

#[allow(dead_code)]
pub fn app_error<V>(app_error_kind: AppErrorKind, error: Box<dyn Error>) -> Result<V, AppError> {
    Err(AppError::new(
        app_error_kind, error.to_string(),
    ))
}

#[allow(dead_code)]
pub fn app_error_with_msg<V>(app_error_kind: AppErrorKind, error_msg: &str) -> Result<V, AppError> {
    Err(AppError::new(
        app_error_kind, error_msg.to_string(),
    ))
}