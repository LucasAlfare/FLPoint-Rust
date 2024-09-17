use serde::{Deserialize, Serialize};

/// Represents user credentials for authentication.
///
/// This struct holds the user's email and plain text password, which
/// should be validated and securely processed before usage.
#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
  /// The user's email address, used as their unique identifier for login.
  pub email: String,

  /// The user's plain text password.
  /// This should be hashed before storage or further processing for security reasons.
  pub plain_password: String,
}

impl Credentials {
  /// Creates a new `Credentials` instance with the provided email and plain text password.
  ///
  /// # Arguments
  ///
  /// * `email` - The email address of the user.
  /// * `plain_password` - The plain text password of the user. This should be securely hashed before usage.
  ///
  /// # Returns
  ///
  /// Returns a new `Credentials` instance containing the provided email and password.
  ///
  /// # Example
  ///
  /// ```
  /// let creds = Credentials::new(String::from("user@example.com"), String::from("password123"));
  /// ```
  pub fn new(email: String, plain_password: String) -> Self {
    // TODO: validate params such as proper email format and password strength
    Credentials { email, plain_password }
  }
}