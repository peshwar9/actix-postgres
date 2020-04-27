use crate::errors::ApiError;
use actix_web::web::{Json};
use crate::utility::{send_json_response};
use std::env;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

pub async fn get_health() -> Result<Json<HealthResponse>,ApiError> {
let health = HealthResponse {
    status: "Ok".to_string(),
    version: env!("CARGO_PKG_VERSION").into(),

};
send_json_response(health)
}