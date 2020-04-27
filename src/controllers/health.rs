use crate::errors::ApiError;
use actix_web::web::{Json};
use crate::utility::{send_json_response};
use std::env;
use serde::{Serialize, Deserialize};
//use log;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_get_health() {
        let response = get_health().await.unwrap();
        assert_eq!(response.into_inner().status, "Ok".to_string());
    }
}