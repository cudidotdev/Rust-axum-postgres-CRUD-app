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
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
  //expose environment variables from .env file
  dotenvy::dotenv().expect("Unable to access .env file");

  //set variables from enviroment variables
  let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");

  //create our database pool
  let db_pool = PgPoolOptions::new()
    .max_connections(64)
    .acquire_timeout(Duration::from_secs(5))
    .connect(&database_url)
    .await
    .expect("can't connect to database");

  //create our tcp listener
  let listener = TcpListener::bind(server_address)
    .await
    .expect("Could not create tcp listener");

  println!("listening on {}", listener.local_addr().unwrap());

  // compose the routes
  let app = Router::new()
    .route("/", get(|| async { "Hello world" }))
    .route("/tasks", get(get_tasks).post(create_task))
    .route("/tasks/:task_id", patch(update_task).delete(delete_task))
    .with_state(db_pool);

  //serve the application
  axum::serve(listener, app)
    .await
    .expect("Error serving application");
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
  let row = sqlx::query_as!(
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
    json!({"success": true, "data": row}).to_string(),
  ))
}

async fn update_task(
  State(db_pool): State<PgPool>,
  Path(task_id): Path<i32>,
  Json(task): Json<UpdateTaskReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
  let mut query = "UPDATE tasks SET task_id = $1".to_owned();

  let mut i = 2;

  if task.name.is_some() {
    query.push_str(&format!(", name = ${i}"));
    i = i + 1;
  };

  if task.priority.is_some() {
    query.push_str(&format!(", priority = ${i}"));
  };

  query.push_str(&format!(" WHERE task_id = $1"));

  let mut s = sqlx::query(&query).bind(task_id);

  if task.name.is_some() {
    s = s.bind(task.name);
  }

  if task.priority.is_some() {
    s = s.bind(task.priority);
  }

  s.execute(&db_pool).await.map_err(|e| {
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
  sqlx::query!("DELETE FROM tasks WHERE task_id = $1", task_id,)
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
