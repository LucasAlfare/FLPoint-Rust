use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequestDTO {
  pub name: String,
  pub plain_password: String,
  pub email: String,
  pub is_admin: bool,
}

impl CreateUserRequestDTO {
  pub fn new(name: String, plain_password: String, email: String, is_admin: bool) -> Self {
    // TODO: validate params
    CreateUserRequestDTO { name, plain_password, email, is_admin }
  }
}