use crate::models::schema::employees;
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

#[derive(Insertable)]
#[table_name = "employees"]
pub struct NewEmployee {
    pub firstname: String,
    pub lastname: String,
    pub department: String,
    pub salary: i32,
    pub age: i32,
}

pub fn create_new_employee(new_emp: NewEmployee) -> Result<Employee, Error> {
   


    let connection = establish_connection();
    let new_employee: Employee = diesel::insert_into(employees::table)
    .values(&new_emp)
    .get_result(&connection)
    .expect("Error saving new post");
    Ok(new_employee)
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

pub fn get_employee(user_id: i32) -> Result<Employee,Error> {
    use schema::employees::dsl::*;
    let connection = establish_connection();

   // let results: Vec<Employee> = vec![];
 let result = employees
        .filter(id.eq(user_id))
        .first::<Employee>(&connection)
   //     .load::<Employee>(&connection)
        .expect("Error loading employees");
    Ok(result)

}