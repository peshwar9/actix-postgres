use crate::controllers::users::{find, find_all, welcome, create_employee, update_employee};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .route("/hello", web::get().to(welcome))
    .route("/hello/{name}", web::get().to(welcome))
    .route("/findall", web::get().to(find_all))
    .route("/find/{id}", web::get().to(find))
    .route("/employee", web::post().to(create_employee))
    .route("/employee/{id}", web::put().to(update_employee));
}