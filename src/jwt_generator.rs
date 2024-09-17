use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

const JWT_SECRET: &'static str = "SECRET!";

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
  pub user_id: String,
  pub is_admin: bool,
  // TODO: expiration
}

pub fn generate_jwt(claims: JwtClaims) -> String {
  encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(JWT_SECRET.as_ref()),
  ).unwrap()
}