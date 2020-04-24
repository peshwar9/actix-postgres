use crate::models::users::{Employee,get_all_employees, get_employee};


use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::error::Error;
use actix_web::web::{Json,Path};
use crate::utility::{send_json_response};
use serde::{Serialize};



#[derive(Serialize)]
pub struct EmployeeResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub department: String,
 //   pub salary: i32,
 //   pub age: i32,
}

#[derive(Serialize)]
pub struct EmployeesResponse(pub Vec<EmployeeResponse>);

impl From<Employee> for EmployeeResponse {
    fn from(emp: Employee) -> Self {
        EmployeeResponse {
            id: emp.id,
            first_name: emp.first_name.to_string(),
            last_name: emp.last_name.to_string(),
            department: emp.department.to_string(),
        }
    }
}

impl From<Vec<Employee>> for EmployeesResponse {
    fn from(emps: Vec<Employee>) -> Self {
        EmployeesResponse(emps.into_iter().map(|emp| emp.into()).collect())
    }
}



pub async fn welcome(request: HttpRequest) -> impl Responder {

    let name = request.match_info().get("name").unwrap_or("World");
    format!("Hello {}", &name)
}

pub async fn find_all() -> Result<Json<EmployeesResponse>,Error> {
    let results = get_all_employees()?;
  
  //HttpResponse::Ok().json(results).await
  send_json_response(results.into())

}


pub async fn find(user_id: Path<i32>) -> Result<Json<EmployeeResponse>,Error> {
  
 let result = get_employee(*user_id)?;
 send_json_response(result.into())
  //HttpResponse::Ok().json(results).await
  }