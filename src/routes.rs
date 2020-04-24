use crate::controllers::users::{find, find_all, welcome};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .route("/hello", web::get().to(welcome))
    .route("/hello/{name}", web::get().to(welcome))
    .route("/findall", web::get().to(find_all))
    .route("/find/{id}", web::get().to(find));
}