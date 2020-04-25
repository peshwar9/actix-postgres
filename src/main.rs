#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};

mod controllers;
mod database;
mod models;
mod routes;
mod utility;
use routes::routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
