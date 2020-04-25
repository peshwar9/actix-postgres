use crate::models::schema::employees;
use serde::{Serialize,Deserialize};
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

#[derive(AsChangeset, Serialize, Deserialize)]
#[table_name = "employees"]
pub struct UpdateEmployee {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub department: String,
}

pub fn create_new_employee(new_emp: NewEmployee) -> Result<Employee, Error> {
   


    let connection = establish_connection();
    let new_employee: Employee = diesel::insert_into(employees::table)
    .values(&new_emp)
    .get_result(&connection)
    .expect("Error saving new post");
    Ok(new_employee)
}

pub fn update_employee_details(exist_emp: UpdateEmployee) -> Result<Employee, Error> {
    use crate::models::schema::employees::dsl::{id,employees};
    let connection = establish_connection();
    let updated_employee: Employee = diesel::update(employees)
                                        .filter(id.eq(exist_emp.id.clone()))
                                        .set(&exist_emp)
                                        .get_result::<Employee>(&connection)
                                        .expect(&format!("Unable to find post id {}",&exist_emp.id));
    Ok(updated_employee)
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