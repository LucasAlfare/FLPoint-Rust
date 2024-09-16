mod credentials;
mod user;
mod create_user_request_dto;
mod data_source;

use crate::create_user_request_dto::CreateUserRequestDTO;
use crate::credentials::Credentials;
use crate::data_source::InMemoryDataSource;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[tokio::main]
async fn main() {
  let root_data_source = Arc::new(InMemoryDataSource::new());

  let app = Router::new()
    .route("/health", get(|| async { "Hello, from Rust API!" }))
    .route("/register", post(register))
    .route("/login", post(login))
    .with_state(root_data_source);

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

// cls; curl -v -X POST http://localhost:3000/register -H "Content-Type: application/json" -d '{ "name": "lucas", "plain_password": "1234567890", "email": "admin@system.com", "is_admin": true }'
async fn register(
  State(data_source): State<Arc<InMemoryDataSource>>,
  Json(payload): Json<CreateUserRequestDTO>,
) -> impl IntoResponse {
  data_source.create_user(payload, false)
}

// cls; curl -v -X POST http://localhost:3000/login -H "Content-Type: application/json" -d '{ "email": "admin@system.com", "plain_password": "1234567890" }'
async fn login(
  State(data_source): State<Arc<InMemoryDataSource>>,
  Json(credentials): Json<Credentials>,
) -> impl IntoResponse {
  data_source.login_user(credentials)
}