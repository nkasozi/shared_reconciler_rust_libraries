use actix_web::HttpResponse;
use serde::Serialize;

use crate::internal::shared_reconciler_rust_libraries::models::entities::app_errors::{AppError, AppErrorKind};

#[allow(dead_code)]
pub fn ok_or_error<T: Serialize>(response: Result<T, AppError>) -> HttpResponse {
    return match response {
        Ok(details) => HttpResponse::Ok().json(details),
        Err(err) => match err.kind {
            AppErrorKind::NotFound => not_found_error(err),
            AppErrorKind::BadClientRequest => bad_client_request(err),
            _ => internal_server_error(err),
        },
    };
}

#[allow(dead_code)]
pub fn internal_server_error(error: AppError) -> HttpResponse {
    return HttpResponse::InternalServerError().json(format!("{}", error));
}

#[allow(dead_code)]
pub fn bad_client_request(error: AppError) -> HttpResponse {
    return HttpResponse::BadRequest().json(format!("{}", error));
}

#[allow(dead_code)]
pub fn not_found_error(error: AppError) -> HttpResponse {
    return HttpResponse::NotFound().json(format!("{}", error));
}