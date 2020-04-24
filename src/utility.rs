use actix_web::web::{Json};
use actix_web::error::Error;

use serde::Serialize;

pub fn send_json_response<T>(data: T) -> Result<Json<T>,Error>
where T: Serialize,
{
    Ok(Json(data))
}
