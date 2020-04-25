use actix_web::{
    body::Body,
    error::Error,
    web::{HttpResponse, Json},
};

use serde::Serialize;

pub fn send_json_response<T>(data: T) -> Result<Json<T>, Error>
where
    T: Serialize,
{
    Ok(Json(data))
}

pub fn respond_ok() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(Body::Empty))
}
