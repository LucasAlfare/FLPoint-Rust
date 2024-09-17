use serde::{Deserialize, Serialize};

/// Data Transfer Object (DTO) for creating a new user.
///
/// This struct is used to encapsulate the data required to create a new user in the system.
/// It includes the user's name, plain text password, email, and admin status.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequestDTO {
  /// The full name of the user.
  pub name: String,

  /// The user's plain text password.
  /// This should be validated and securely hashed before storing it.
  pub plain_password: String,

  /// The user's email address.
  /// This serves as a unique identifier and should be validated for proper format.
  pub email: String,

  /// Specifies whether the new user will have administrative privileges.
  pub is_admin: bool,
}

impl CreateUserRequestDTO {
  /// Creates a new `CreateUserRequestDTO` instance with the provided details.
  ///
  /// # Arguments
  ///
  /// * `name` - The full name of the user.
  /// * `plain_password` - The plain text password of the user. Should be validated for strength.
  /// * `email` - The email address of the user. Must be unique and follow a valid format.
  /// * `is_admin` - A boolean value indicating whether the user should be an admin.
  ///
  /// # Returns
  ///
  /// Returns a new `CreateUserRequestDTO` populated with the given information.
  ///
  /// # Example
  ///
  /// ```
  /// let create_user_dto = CreateUserRequestDTO::new(
  ///     String::from("John Doe"),
  ///     String::from("securepassword123"),
  ///     String::from("john.doe@example.com"),
  ///     false
  /// );
  /// ```
  pub fn new(name: String, plain_password: String, email: String, is_admin: bool) -> Self {
    // TODO: validate params such as proper email format and password strength
    CreateUserRequestDTO { name, plain_password, email, is_admin }
  }
}