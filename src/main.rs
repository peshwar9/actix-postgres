#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
use actix_web::{App, HttpServer,middleware::Logger};

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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //Create connection pool
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {

        App::new()
            .wrap(Logger::default())
            .data(establish_connection().clone())
            .configure(routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
