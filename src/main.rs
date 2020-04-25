#[macro_use]
extern crate diesel;
use diesel::r2d2::{self, ConnectionManager};

use actix_web::{App, HttpServer};
use std::env;
use diesel::pg::PgConnection;
mod controllers;
mod database;
mod models;
mod routes;
mod utility;
use routes::routes;
use dotenv::dotenv;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set");
    //Create connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
                .build(manager)
                .expect("Failed to create pool");

    HttpServer::new( move || App::new()
                .data(pool.clone())
                .configure(routes))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
