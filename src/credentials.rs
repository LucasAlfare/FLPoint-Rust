use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
  pub email: String,
  pub plain_password: String,
}

impl Credentials {
  pub fn new(email: String, plain_password: String) -> Self {
    // TODO: validate params
    Credentials { email, plain_password }
  }
}