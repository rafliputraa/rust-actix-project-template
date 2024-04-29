use actix_web::HttpResponse;
use actix_web::web::Json;
use serde::Serialize;
use crate::errors::ApiError;

pub fn respond_json<T>(data: T) -> Result<Json<T>, ApiError>
    where
        T: Serialize,
{
    Ok(Json(data))
}

/// Helper function to reduce boilerplate of an empty OK response
pub fn respond_ok() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().finish())
}