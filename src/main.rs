#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
use actix_cors::Cors;
use actix_web::{http, middleware::Logger, App, HttpServer};

mod auth;
mod controllers;
mod database;
mod envdata;
mod errors;
mod models;
mod routes;
mod utility;

use database::establish_connection;
use envdata::ENVDATA;
use routes::routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    .allowed_origin(&ENVDATA.cors_origin)
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .data(establish_connection().clone())
            .configure(routes)
    })
    .bind(&ENVDATA.server)?
    .run()
    .await
}
