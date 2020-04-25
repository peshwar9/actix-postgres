use crate::models::users::{
    create_new_employee, delete_employee, get_all_employees, get_employee, update_employee_details,
    Employee, NewEmployee, UpdateEmployee,
};

use crate::utility::{respond_ok, send_json_response};
use actix_web::error::Error;
use actix_web::web::{Json, Path};
use actix_web::{HttpRequest, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::{Pool};

#[derive(Serialize, Deserialize)]
pub struct CreateEmployeeRequest {
    pub first_name: String,
    pub last_name: String,
    pub department: String,
    pub salary: i32,
    pub age: i32,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateEmployeeRequest {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub department: String,
}

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

// Create new employee

pub async fn create_employee(
    params: Json<CreateEmployeeRequest>,
) -> Result<Json<EmployeeResponse>, Error> {
    // let emp = serde_json::from_str(&params);
    let new_emp = NewEmployee {
        firstname: params.first_name.to_string(),
        lastname: params.last_name.to_string(),
        department: params.department.to_string(),
        salary: params.salary,
        age: params.age,
    };
    let new_employee: Employee = create_new_employee(new_emp)?;

    send_json_response(new_employee.into())
}

pub async fn update_employee(
    params: Json<UpdateEmployeeRequest>,
) -> Result<Json<EmployeeResponse>, Error> {
    let exist_emp = UpdateEmployee {
        id: params.id,
        firstname: params.first_name.to_string(),
        lastname: params.last_name.to_string(),
        department: params.department.to_string(),
    };
    let updated_employee: Employee = update_employee_details(exist_emp)?;

    send_json_response(updated_employee.into())
}

pub async fn delete(user_id: Path<i32>) -> Result<HttpResponse, Error> {
    delete_employee(*user_id)?;
    respond_ok()
}

pub async fn find_all() -> Result<Json<EmployeesResponse>, Error> {
    let results = get_all_employees()?;

    //HttpResponse::Ok().json(results).await
    send_json_response(results.into())
}

pub async fn find(db: web::Data<Pool>, user_id: Path<i32>) -> Result<Json<EmployeeResponse>, Error> {
    let result = get_employee(db, *user_id)?;
    send_json_response(result.into())
    //HttpResponse::Ok().json(results).await
}
