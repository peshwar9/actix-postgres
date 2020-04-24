use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer,Responder};
pub mod model; 
use model::{Employee};

async fn welcome(request: HttpRequest) -> impl Responder {
    let name = request.match_info().get("name").unwrap_or("World");
    format!("Hello {}", &name)
}

async fn find_all() -> impl Responder {
    HttpResponse::Ok().json(vec![
        Employee {
            id: 1,
            first_name: "Bob".to_string(),
            last_name:"Marley".to_string(),
            department: "Lyrics".to_string(),
            salary: 4000,
            age: 32,
        },
        Employee {
            id: 2,
            first_name: "Lionel".to_string(),
            last_name:"Ritchie".to_string(),
            department: "Vocals".to_string(),
            salary: 4200,
            age: 38,            
        }
    ])
}

async fn find() -> impl Responder {
    HttpResponse::Ok().json(
        Employee {
            id: 2,
            first_name: "Lionel".to_string(),
            last_name:"Ritchie".to_string(),
            department: "Vocals".to_string(),
            salary: 4200,
            age: 38,                
        }
    )
}
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
            .route("/hello", web::get().to(welcome))
            .route("/hello/{name}", web::get().to(welcome))
            .route("/findall", web::get().to(find_all))
            .route("/find", web::get().to(find))
    } )
    .bind("127.0.0.1:8000")?
    .run()
    .await

   
}