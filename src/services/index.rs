use crate::handlers::response::{error::ApiErrorResponse, ok::ApiOkResponse};
use actix_web::{http::StatusCode, HttpResponse};

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().json(ApiOkResponse::new().status(StatusCode::OK).message("online").build())
}

/// Default service to display if the endpoint does not exist.
pub async fn oops() -> HttpResponse {
    HttpResponse::NotFound().json(ApiErrorResponse::new().status(StatusCode::NOT_FOUND).message("Not Found").build())
}
