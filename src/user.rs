use serde::{Deserialize, Serialize};

/// Represents a user in the system.
///
/// This struct contains information about a user, including
/// their unique ID, name, hashed password, email, and admin status.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
  /// The unique identifier for the user.
  pub id: usize,

  /// The full name of the user.
  pub name: String,

  /// The hashed password of the user for authentication.
  /// This should be securely hashed and not stored in plaintext.
  pub hashed_password: String,

  /// The unique email address of the user.
  /// This serves as a contact point and must be unique within the system.
  pub email: String,

  /// Indicates whether the user has administrative privileges.
  /// Admin users have access to higher-level system functionalities.
  pub is_admin: bool,
}

impl User {
  /// Creates a new `User` instance with the provided details.
  ///
  /// # Arguments
  ///
  /// * `id` - The unique identifier for the user.
  /// * `name` - The full name of the user.
  /// * `hashed_password` - The hashed password of the user.
  /// * `email` - The unique email address of the user.
  /// * `is_admin` - A boolean indicating if the user is an admin.
  ///
  /// # Returns
  ///
  /// Returns a new `User` instance populated with the given data.
  ///
  /// # Example
  ///
  /// ```
  /// let user = User::new(1, String::from("Alice"), String::from("hashed_pw"), String::from("alice@example.com"), false);
  /// ```
  pub fn new(id: usize, name: String, hashed_password: String, email: String, is_admin: bool) -> Self {
    User { id, name, hashed_password, email, is_admin }
  }
}