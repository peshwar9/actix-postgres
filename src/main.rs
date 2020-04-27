#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
use actix_web::{App, http, HttpServer,middleware::Logger};
use actix_cors::Cors;

mod controllers;
mod database;
mod errors;
mod models;
mod routes;
mod utility;
mod auth;

use routes::routes;
use database::establish_connection;
use dotenv::dotenv;
use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //Create connection pool
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {

        App::new()
            .wrap(Logger::default())
            .wrap(            
            Cors::new() // <- Construct CORS middleware builder
            .allowed_origin(&env::var("CORS_ORIGIN").unwrap())
            .allowed_methods(vec!["GET", "POST","PUT","DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600)
            .finish())
            .data(establish_connection().clone())
            .configure(routes)
    })
    .bind(&env::var("SERVER").unwrap())?
    .run()
    .await
}
