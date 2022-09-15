use actix_web::HttpResponse;
use serde::Serialize;
use crate::internal::shared_reconciler_rust_libraries::models::entities::app_errors::{AppError, AppErrorKind};

pub fn ok_or_error<T: Serialize> (response:Result<T,AppError>) -> HttpResponse{
    return match response {
        Ok(details) => HttpResponse::Ok().json(details),
        Err(err) => match err.kind {
            AppErrorKind::NotFound => HttpResponse::NotFound().json(format!("{}", err)),
            AppErrorKind::BadClientRequest => HttpResponse::BadRequest().json(format!("{}", err)),
            _ => HttpResponse::InternalServerError().json(format!("{}", err)),
        },
    };
}