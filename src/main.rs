use std::time::Duration;

use axum::{
  extract::{Path, State},
  http::StatusCode,
  routing::{get, patch},
  Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[tokio::main]
async fn main() {
  //expose environment variables from .env file
  dotenvy::dotenv().expect("Unable to access .env file");

  //set variables from enviroment variables
  let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");

  //create our database pool
  let db_pool = PgPoolOptions::new()
    .max_connections(16)
    .acquire_timeout(Duration::from_secs(5))
    .connect(&database_url)
    .await
    .expect("can't connect to database");

  //create our tcp listener
  let listener = tokio::net::TcpListener::bind(server_address).await.unwrap();

  // compose the routes
  let app = Router::new()
    .route("/tasks", get(get_tasks).post(create_task))
    .route("/tasks/:task_id", patch(update_task).delete(delete_task))
    .with_state(db_pool);

  println!("listening on {}", listener.local_addr().unwrap());

  axum::serve(listener, app).await.unwrap();
}

async fn get_tasks(
  State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
  let rows = sqlx::query_as!(TaskRow, "SELECT * FROM tasks ORDER BY task_id")
    .fetch_all(&db_pool)
    .await
    .map_err(|e| {
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "message": e.to_string()}).to_string(),
      )
    })?;

  Ok((
    StatusCode::OK,
    json!({"success": true, "data": rows}).to_string(),
  ))
}

async fn create_task(
  State(db_pool): State<PgPool>,
  Json(task): Json<CreateTaskReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
  let rows = sqlx::query_as!(
    CreateTaskRow,
    "INSERT INTO tasks (name, priority) VALUES ($1, $2) RETURNING task_id",
    task.name,
    task.priority
  )
  .fetch_one(&db_pool)
  .await
  .map_err(|e| {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      json!({"success": false, "message": e.to_string()}).to_string(),
    )
  })?;

  Ok((
    StatusCode::CREATED,
    json!({"success": true, "data": rows}).to_string(),
  ))
}

async fn update_task(
  State(db_pool): State<PgPool>,
  Path(task_id): Path<i32>,
  Json(task): Json<UpdateTaskReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
  sqlx::query_as!(
    CreateTask,
    "WITH t AS (SELECT * FROM tasks WHERE task_id = $1)
     UPDATE tasks SET
      name = CASE WHEN $2 = 'X-SKIP' THEN (SELECT name FROM t) ELSE $2 END,  
      priority = CASE WHEN $3 = -1 THEN (SELECT priority FROM t) WHEN $3 = 0 THEN NULL ELSE $3 END  
      WHERE task_id = $1",
    task_id,
    task.name.unwrap_or("X-SKIP".to_owned()),
    task.priority.unwrap_or(-1)
  )
  .execute(&db_pool)
  .await
  .map_err(|e| {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      json!({"success": false, "message": e.to_string()}).to_string(),
    )
  })?;

  Ok((StatusCode::OK, json!({"success":true}).to_string()))
}

async fn delete_task(
  State(db_pool): State<PgPool>,
  Path(task_id): Path<i32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
  sqlx::query_as!(CreateTask, "DELETE FROM tasks WHERE task_id = $1", task_id,)
    .execute(&db_pool)
    .await
    .map_err(|e| {
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "message": e.to_string()}).to_string(),
      )
    })?;

  Ok((StatusCode::OK, json!({"success":true}).to_string()))
}

#[derive(Serialize)]
struct TaskRow {
  task_id: i32,
  name: String,
  priority: Option<i32>,
}

#[derive(Deserialize)]
struct CreateTaskReq {
  name: String,
  priority: Option<i32>,
}

#[derive(Serialize)]
struct CreateTaskRow {
  task_id: i32,
}

#[derive(Deserialize)]
struct UpdateTaskReq {
  name: Option<String>,
  priority: Option<i32>,
}
