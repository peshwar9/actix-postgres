#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};

mod controllers;
mod database;
mod models;
mod routes;
mod utility;
use routes::routes;

use database::establish_connection;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //Create connection pool

    HttpServer::new(move || {
        App::new()
            .data(establish_connection().clone())
            .configure(routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
