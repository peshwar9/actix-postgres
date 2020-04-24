#[macro_use] extern crate diesel;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer,Responder};
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub mod schema;
pub mod model; 

use dotenv::dotenv;
use std::env;

use model::{Employee};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

async fn welcome(request: HttpRequest) -> impl Responder {

    let name = request.match_info().get("name").unwrap_or("World");
    format!("Hello {}", &name)
}

/*async fn find_all() -> impl Responder {
/*
    use schema::employees::dsl::*;
    let connection = establish_connection();
    
let results = employees
.limit(5)
.load::<Employee>(&connection)
.expect("Error loading employees");
    
HttpResponse::Ok().json(results);
*/
HttpResponse::Ok().json({"msg":"JGD"});
}
*/
async fn find() -> Result<HttpResponse,actix_http::error::Error> {
  /*  HttpResponse::Ok().json(
        Employee {
            id: 2,
            first_name: "Lionel".to_string(),
            last_name:"Ritchie".to_string(),
            department: "Vocals".to_string(),
            salary: 4200,
            age: 38,                
        }
        
    )*/

    use schema::employees::dsl::*;
    let connection = establish_connection();

   // let results: Vec<Employee> = vec![];
 let results = employees
.limit(5)
.load::<Employee>(&connection)
.expect("Error loading employees");
    
HttpResponse::Ok().json(results).await
}
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
            .route("/hello", web::get().to(welcome))
            .route("/hello/{name}", web::get().to(welcome))
       //     .route("/findall", web::get().to(find_all))
            .route("/find", web::get().to(find))
    } )
    .bind("127.0.0.1:8000")?
    .run()
    .await

   
}