use crate::database::Pool;
use crate::models::schema;
use crate::models::schema::employees;
use actix_web::{error::Error, web};
use diesel::prelude::*;
use schema::employees::dsl::*;

#[derive(Queryable)]
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

#[derive(AsChangeset)]
#[table_name = "employees"]
pub struct UpdateEmployee {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub department: String,
}

pub fn create_new_employee(pool: web::Data<Pool>, new_emp: NewEmployee) -> Result<Employee, Error> {
    let connection = pool.get().unwrap();
    let new_employee: Employee = diesel::insert_into(employees::table)
        .values(&new_emp)
        .get_result(&connection)
        .expect("Error saving new post");
    Ok(new_employee)
}

pub fn update_employee_details(
    pool: web::Data<Pool>,
    exist_emp: UpdateEmployee,
) -> Result<Employee, Error> {
    let connection = pool.get().unwrap();
    let updated_employee: Employee = diesel::update(employees)
        .filter(id.eq(exist_emp.id.clone()))
        .set(&exist_emp)
        .get_result::<Employee>(&connection)
        .expect(&format!("Unable to find post id {}", &exist_emp.id));
    Ok(updated_employee)
}

pub fn get_all_employees(pool: web::Data<Pool>) -> Result<Vec<Employee>, Error> {
    let connection = pool.get().unwrap();

    let results = employees
        .limit(10)
        .load::<Employee>(&connection)
        .expect("Error loading employees");
    Ok(results)
}

pub fn get_employee(pool: web::Data<Pool>, user_id: i32) -> Result<Employee, Error> {
    //  let connection = establish_connection();
    let connection = pool.get().unwrap();
    let result = employees
        .filter(id.eq(user_id))
        .first::<Employee>(&connection)
        .expect("Error loading employees");
    Ok(result)
}

pub fn delete_employee(pool: web::Data<Pool>, user_id: i32) -> Result<(), Error> {
    let connection = pool.get().unwrap();
    diesel::delete(employees)
        .filter(id.eq(user_id))
        .execute(&connection)
        .expect("Error in deleting employee");

    Ok(())
}
