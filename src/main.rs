#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};

mod controllers;
mod database;
mod errors;
mod models;
mod routes;
mod utility;
mod auth;

use routes::routes;
use auth::{validator};
use actix_web_httpauth::middleware::HttpAuthentication;
use database::establish_connection;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //Create connection pool

    HttpServer::new(move || {

        let auth = HttpAuthentication::bearer(validator);        
        App::new()
            .data(establish_connection().clone())
            .wrap(auth)
            .configure(routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
