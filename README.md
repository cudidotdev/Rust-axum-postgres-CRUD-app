# Axum-Postgres CRUD App

Hi, welcome to Axum-Postgres, a basic CRUD (Create, Read, Update, Delete) application built with Rust's Axum web framework and PostgreSQL. This project provides a simple API for managing tasks, demonstrating the integration of Axum with a PostgreSQL database.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Setup](#setup)
- [Database Configuration](#database-configuration)
- [Run the Application](#run-the-application)
- [API Endpoints](#api-endpoints)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)

## Prerequisites

Make sure you have the following installed on your system:

- Rust (https://www.rust-lang.org/tools/install)
- PostgreSQL (https://www.postgresql.org/download/)

## Setup

1. Clone the repository:

```bash
git clone https://github.com/CudiLala/Rust-axum-postgres-CRUD-app.git
cd Rust-axum-postgres-CRUD-app

```

2. Build binaries
   
```bash
cargo build
```

3. Database Configuration

Create a PostgreSQL database and user by executing the following SQL commands in your PostgreSQL shell or client:

```sql
-- create user
CREATE ROLE axum_postgres WITH LOGIN PASSWORD 'axum_postgres';

-- create database
CREATE DATABASE axum_postgres WITH OWNER = 'axum_postgres';

-- in your axum_postgres database
-- create task table
CREATE TABLE tasks (
  task_id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  priority INT
);

```

Copy the contents of the `create.sql` file and execute it in your PostgreSQL database.

Create a `.env` file in the project root and configure the `DATABASE_URL` and `SERVER_ADDRESS`:

```env
DATABASE_URL=postgres://axum_postgres:axum_postgres@127.0.0.1:5432/axum_postgres
SERVER_ADDRESS=127.0.0.1:7878

```

4. Run the application
Run the application with
```bash
cargo run
```

## API Endpoints

### Get all tasks 
```http
GET /tasks

```
Retrieves a list of all tasks.

### Create task
```http
POST /tasks
Content-Type: application/json

{
  "name": "Task Name",
  "priority": 1
}

```
Creates a new task.

### Update task
```http
PATCH /tasks/{task_id}
Content-Type: application/json

{
  "name": "New Task Name",
  "priority": 2
}

```
Updates an existing task.

### Delete task
```http
DELETE /tasks/{task_id}

```

## Contributing
Feel free to contribute by opening issues or creating pull requests. Your feedback and contributions are highly appreciated.

## License
This project is licensed under the CC0 License.
