use actix_web::{
    body::Body,
    web::{HttpResponse, Json},
};
use crate::errors::ApiError;

use serde::Serialize;

pub fn send_json_response<T>(data: T) -> Result<Json<T>, ApiError>
where
    T: Serialize,
{
    Ok(Json(data))
}

pub fn respond_ok(data: &'static str) -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().body(data))
}
