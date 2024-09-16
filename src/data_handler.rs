use crate::create_point_request_dto::CreatePointRequestDTO;
use crate::create_user_request_dto::CreateUserRequestDTO;
use crate::credentials::Credentials;
use crate::point::Point;
use crate::user::User;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Mutex;

#[derive(Debug)]
pub struct InMemoryDataHandler {
  pub users: Mutex<Vec<User>>,
  pub points: Mutex<Vec<Point>>,
}

impl InMemoryDataHandler {
  pub fn new() -> InMemoryDataHandler {
    InMemoryDataHandler {
      users: Mutex::new(Vec::new()),
      points: Mutex::new(Vec::new()),
    }
  }

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

  // TODO: should be under an authenticated middleware
  pub fn create_point(&self, dto: CreatePointRequestDTO) -> impl IntoResponse {
    let mut local_points = self.points.lock().unwrap();
    let next_id = local_points.len() + 1;
    local_points.push(Point::new(next_id, dto.related_user_id, dto.instant));

    println!("{:?}", local_points);

    (StatusCode::CREATED, "Point created")
  }
}