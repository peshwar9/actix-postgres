use crate::database::Pool;
use crate::models::schema;
use crate::models::schema::employees;
use actix_web::web;
use diesel::prelude::*;
use schema::employees::dsl::*;
use crate::errors::ApiError;

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

pub fn create_new_employee(pool: web::Data<Pool>, new_emp: NewEmployee) -> Result<Employee, ApiError> {
    let connection = pool.get().unwrap();
    let not_found = format!("Unable to create new employee");

    let new_employee: Employee = diesel::insert_into(employees::table)
        .values(&new_emp)
        .get_result(&connection)
        .map_err(|_| ApiError::NotFound(not_found))?;
    Ok(new_employee)
}

pub fn update_employee_details(
    pool: web::Data<Pool>,
    exist_emp: UpdateEmployee,
) -> Result<Employee, ApiError> {
    let connection = pool.get().unwrap();
    let not_found = format!("There is no such employee {} in your company",&exist_emp.id);
    let updated_employee: Employee = diesel::update(employees)
        .filter(id.eq(exist_emp.id.clone()))
        .set(&exist_emp)
        .get_result::<Employee>(&connection)
        .map_err(|_| ApiError::NotFound(not_found))?;
    Ok(updated_employee)
}

pub fn get_all_employees(pool: web::Data<Pool>) -> Result<Vec<Employee>, ApiError> {
    let connection = pool.get().unwrap();
    let not_found = format!("There are no employees in your company");

    let results = employees
        .limit(10)
        .load::<Employee>(&connection)
        .map_err(|_| ApiError::NotFound(not_found))?;
    Ok(results)
}

pub fn get_employee(pool: web::Data<Pool>, user_id: i32) -> Result<Employee, ApiError> {
    //  let connection = establish_connection();
    let not_found = format!("Employee {} is not found, maybe he left", user_id);
    let connection = pool.get().unwrap();
    let result = employees
        .filter(id.eq(user_id))
        .first::<Employee>(&connection)
        .map_err(|_| ApiError::NotFound(not_found))?;
    Ok(result)
}

pub fn delete_employee(pool: web::Data<Pool>, user_id: i32) -> Result<(), ApiError> {
    let connection = pool.get().unwrap();
    let not_found = format!("Employee {} is not found, unable to delete what I cannot find", user_id);
    diesel::delete(employees)
        .filter(id.eq(user_id))
        .execute(&connection)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(())
}
