# create user
CREATE ROLE axum_postgres WITH LOGIN PASSWORD 'axum_postgres';

#create database
CREATE DATABASE axum_postgres WITH OWNER = 'axum_postgres';

# create task table
CREATE TABLE tasks (
  task_id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  priority INT
);