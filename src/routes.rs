use crate::controllers::users::{
    create_employee, delete, find, find_all, update_employee, welcome, get_token,
};
use crate::controllers::health::{get_health};
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::auth::{validator};

pub fn routes(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);        

    cfg
        .route("/health", web::get().to(get_health))
        .route("/auth", web::get().to(get_token))
        .route("/hello", web::get().to(welcome))
        .route("/hello/{name}", web::get().to(welcome))
        .service(
            web::scope("/employee")
                .wrap(auth)
                .route("/findall", web::get().to(find_all))
                .route("/find/{id}", web::get().to(find))
                .route("/", web::post().to(create_employee))
                .route("/{id}", web::delete().to(delete))
                .route("/{id}", web::put().to(update_employee)),
        );


        
}
