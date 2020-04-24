-- Your SQL goes here
CREATE TABLE employees (
    id SERIAL PRIMARY KEY,
    firstname VARCHAR not NULL,
    lastname VARCHAR not NULL,
    department VARCHAR not NULL,
    salary INT not NULL,
    age INT not NULL
)