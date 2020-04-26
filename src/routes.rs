use crate::controllers::users::{
    create_employee, delete, find, find_all, update_employee, welcome, get_token,
};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/hello", web::get().to(welcome))
        .route("/hello/{name}", web::get().to(welcome))
        .route("/findall", web::get().to(find_all))
        .route("/find/{id}", web::get().to(find))
        .route("/employee", web::post().to(create_employee))
        .route("/employee/{id}", web::delete().to(delete))
        .route("/employee/{id}", web::put().to(update_employee))
        .route("/auth", web::post().to(get_token));
}
