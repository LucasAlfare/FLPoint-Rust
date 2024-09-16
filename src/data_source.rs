use crate::create_user_request_dto::CreateUserRequestDTO;
use crate::credentials::Credentials;
use crate::user::User;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Mutex;

#[derive(Debug)]
pub struct InMemoryDataSource {
  pub users: Mutex<Vec<User>>,
}

impl InMemoryDataSource {
  pub fn new() -> InMemoryDataSource {
    InMemoryDataSource {
      users: Mutex::new(Vec::new()),
    }
  }

  // cls; curl -v -X POST http://localhost:3000/register -H "Content-Type: application/json" -d '{ "name": "lucas", "plain_password": "1234567890", "email": "admin@system.com", "is_admin": true }'
  pub fn create_user(
    &self,
    dto: CreateUserRequestDTO,
    is_admin: bool,
  ) -> impl IntoResponse {
    let mut local_users = self.users.lock().unwrap();

    if local_users.iter().any(|u: &User| { return (u.email == dto.email); }) {
      return (StatusCode::BAD_REQUEST, "User email already exists");
    }

    let count = local_users.len();
    local_users.push(User::new(
      count + 1,
      dto.name,
      dto.plain_password,
      dto.email,
      is_admin,
    ));

    (StatusCode::CREATED, "User created")
  }

  // cls; curl -v -X POST http://localhost:3000/login -H "Content-Type: application/json" -d '{ "email": "admin@system.com", "plain_password": "1234567890" }'
  pub fn login_user(&self, credentials: Credentials) -> impl IntoResponse {
    let mut local_users = self.users.lock().unwrap();
    let mut found_user = local_users.iter().find(|u| u.email == credentials.email);
    match found_user {
      None => { (StatusCode::BAD_REQUEST, "User email not found") }
      Some(_) => {
        // TODO: check with hashing match
        if found_user.unwrap().hashed_password != credentials.plain_password {
          return (StatusCode::BAD_REQUEST, "Email or password doesn't match");
        }

        (StatusCode::OK, "User logged in! JWT: eySODFJ98ehfauiehfPIHEfhyh")
      }
    }
  }
}