
use serde::{Serialize};
use crate::database::{establish_connection};
use actix_web::error::Error;
use crate::models::schema;
use diesel::prelude::*;

#[derive(Queryable, Serialize)]
pub struct Employee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub department: String,
    pub salary: i32,
    pub age: i32,
}

pub fn get_all_employees() -> Result<Vec<Employee>,Error> {
    use schema::employees::dsl::*;
    let connection = establish_connection();

   // let results: Vec<Employee> = vec![];
 let results = employees
        .limit(5)
        .load::<Employee>(&connection)
        .expect("Error loading employees");
    Ok(results)

}

pub fn get_employee() -> Result<Employee,Error> {
    use schema::employees::dsl::*;
    let connection = establish_connection();

   // let results: Vec<Employee> = vec![];
 let result = employees
        .first::<Employee>(&connection)
   //     .load::<Employee>(&connection)
        .expect("Error loading employees");
    Ok(result)

}