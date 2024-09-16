mod credentials;
mod user;
mod create_user_request_dto;
mod data_handler;
mod point;
mod create_point_request_dto;

use crate::create_point_request_dto::CreatePointRequestDTO;
use crate::create_user_request_dto::CreateUserRequestDTO;
use crate::credentials::Credentials;
use crate::data_handler::InMemoryDataHandler;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{Offset, TimeZone, Timelike};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[tokio::main]
async fn main() {
  let data_handler = Arc::new(InMemoryDataHandler::new());

  let app = Router::new()
    .route("/health", get(|| async { "Hello, from Rust API!" }))
    .route("/register", post(register))
    .route("/login", post(login))
    .route("/point", post(point))
    .with_state(data_handler);

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

// cls; curl -v -X POST http://localhost:3000/register -H "Content-Type: application/json" -d '{ "name": "lucas", "plain_password": "1234567890", "email": "admin@system.com", "is_admin": true }'
async fn register(
  State(handler): State<Arc<InMemoryDataHandler>>,
  Json(payload): Json<CreateUserRequestDTO>,
) -> impl IntoResponse {
  handler.create_user(payload, false)
}

// cls; curl -v -X POST http://localhost:3000/login -H "Content-Type: application/json" -d '{ "email": "admin@system.com", "plain_password": "1234567890" }'
async fn login(
  State(handler): State<Arc<InMemoryDataHandler>>,
  Json(credentials): Json<Credentials>,
) -> impl IntoResponse {
  handler.login_user(credentials)
}

// cls; curl -v -X POST http://localhost:3000/point -H "Content-Type: application/json" -d '{"related_user_id": 1, instant: "2024-09-16T20:37:11.163961900-03:00"}'
async fn point(
  State(handler): State<Arc<InMemoryDataHandler>>,
  Json(dto): Json<CreatePointRequestDTO>,
) -> impl IntoResponse {
  handler.create_point(dto)
}