use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
  pub id: usize,
  pub name: String,
  pub hashed_password: String,
  pub email: String, // unique
  pub is_admin: bool,
}

impl User {
  pub fn new(id: usize, name: String, hashed_password: String, email: String, is_admin: bool) -> Self {
    User { id, name, hashed_password, email, is_admin }
  }
}