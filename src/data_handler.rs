use crate::create_point_request_dto::CreatePointRequestDTO;
use crate::create_user_request_dto::CreateUserRequestDTO;
use crate::credentials::Credentials;
use crate::jwt_generator::{generate_jwt, JwtClaims};
use crate::point::Point;
use crate::point_usecases_rules::{is_within_time_range, passed_at_least_30_min_from_last};
use crate::user::User;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Mutex;

/// A handler for managing in-memory data for users and points.
///
/// This struct contains collections of users and points that are stored
/// in memory using a `Mutex` for thread-safe access. It provides methods
/// for creating users, handling user logins, and creating points, ensuring
/// data integrity and business rules are followed.
#[derive(Debug)]
pub struct InMemoryDataHandler {
  /// A thread-safe collection of users.
  pub users: Mutex<Vec<User>>,
  /// A thread-safe collection of points (punch clock records).
  pub points: Mutex<Vec<Point>>,
}

impl InMemoryDataHandler {
  /// Creates a new instance of `InMemoryDataHandler` with empty user and point lists.
  ///
  /// # Returns
  ///
  /// A new `InMemoryDataHandler` instance.
  pub fn new() -> InMemoryDataHandler {
    InMemoryDataHandler {
      users: Mutex::new(Vec::new()),
      points: Mutex::new(Vec::new()),
    }
  }

  /// Registers a new user based on the provided `CreateUserRequestDTO`.
  ///
  /// This method checks if the email is unique and adds the new user if the conditions are met.
  ///
  /// # Arguments
  ///
  /// * `dto` - The data transfer object containing the user's information.
  /// * `is_admin` - A boolean indicating whether the new user should be an administrator.
  ///
  /// # Returns
  ///
  /// An HTTP response with `StatusCode::CREATED` and the new user's ID if the operation is successful.
  /// If the email is already in use, returns `StatusCode::BAD_REQUEST` and an error message.
  pub fn create_user(
    &self,
    dto: CreateUserRequestDTO,
    is_admin: bool,
  ) -> impl IntoResponse {
    let mut local_users = self.users.lock().unwrap();

    // Check if the email is already registered (must be unique).
    if local_users.iter().any(|u| u.email == dto.email) {
      return (StatusCode::BAD_REQUEST, "User email already exists".to_string());
    }

    // Assign the next available ID and add the new user to the collection.
    let next_id = local_users.len() + 1;
    local_users.push(User::new(
      next_id,
      dto.name,
      dto.plain_password,
      dto.email,
      is_admin,
    ));

    (StatusCode::CREATED, next_id.to_string())
  }

  /// Handles user login by validating credentials.
  ///
  /// This method checks if the email exists in the user list, and if so, verifies the password.
  /// Upon successful authentication, a JWT token is generated for the user.
  ///
  /// # Arguments
  ///
  /// * `credentials` - The user's login credentials (email and password).
  ///
  /// # Returns
  ///
  /// An HTTP response with `StatusCode::OK` and the generated JWT token if the login is successful.
  /// If the email doesn't exist or the password doesn't match, returns `StatusCode::BAD_REQUEST` and an error message.
  pub fn login_user(&self, credentials: Credentials) -> impl IntoResponse {
    let local_users = self.users.lock().unwrap();

    // Find the user by email.
    let found_user = local_users.iter().find(|u| u.email == credentials.email);
    match found_user {
      // If no user is found, return an error.
      None => (StatusCode::BAD_REQUEST, "User doesn't exist".to_string()),
      Some(user) => {
        // TODO: Verify password using a hashed password comparison.
        if user.hashed_password != credentials.plain_password {
          return (StatusCode::BAD_REQUEST, "Email or password doesn't match".to_string());
        }

        // Generate a JWT token for the authenticated user.
        let next_jwt = generate_jwt(JwtClaims {
          user_id: user.id.to_string(),
          is_admin: user.is_admin,
        });
        (StatusCode::OK, next_jwt)
      }
    }
  }

  /// Creates a new point (punch clock record) for a user based on the provided `CreatePointRequestDTO`.
  ///
  /// This method checks if the timestamp falls within the allowed range and whether at least 30 minutes
  /// have passed since the user's last point.
  ///
  /// # Arguments
  ///
  /// * `dto` - The data transfer object containing the point information.
  /// * `jwt` - The JWT token of the user, used for authentication.
  ///
  /// # Returns
  ///
  /// An HTTP response with `StatusCode::CREATED` and created point ID if the point is created successfully.
  /// Returns `StatusCode::BAD_REQUEST` if the time range or other validation rules are not satisfied.
  pub fn create_point(&self, dto: CreatePointRequestDTO, jwt: String) -> impl IntoResponse {
    // Check if the time of the point is within the allowed range.
    if !is_within_time_range(dto.instant) {
      return (StatusCode::BAD_REQUEST, "Instant range not allowed".to_string());
    }

    // TODO: Validate if the user in the JWT matches the user in the DTO.

    let mut local_points = self.points.lock().unwrap();

    // Get all points related to the specified user.
    let related_user_points = local_points.iter().filter(|u| u.id == dto.related_user_id);

    // Check if there are existing points and if 30 minutes have passed since the last one.
    if related_user_points.clone().count() != 0 {
      if !passed_at_least_30_min_from_last(related_user_points.last().unwrap().instant, dto.instant) {
        return (StatusCode::BAD_REQUEST, "Instant range not satisfied".to_string());
      }
    }

    // Assign the next available ID and add the new point to the collection.
    let next_id = local_points.len() + 1;
    local_points.push(Point::new(next_id, dto.related_user_id, dto.instant));
    (StatusCode::CREATED, next_id.to_string())
  }
}