use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

/// Secret key used for signing the JWT.
/// This constant is used as the secret key for generating JWTs.
///
/// **Note:** In a production environment, this value should be securely stored and not hard-coded.
const JWT_SECRET: &'static str = "SECRET!";

/// Represents the claims that will be included in the JWT (JSON Web Token).
///
/// The claims consist of the user ID and the admin status, which will be encoded
/// into the JWT payload. You may extend this struct to include other claims, such as
/// token expiration time.
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
  /// The unique identifier of the user.
  pub user_id: String,

  /// Boolean flag indicating if the user has administrative privileges.
  pub is_admin: bool,

  // TODO: Add expiration time to the claims for enhanced security.
}

/// Generates a JSON Web Token (JWT) based on the provided claims.
///
/// The function uses the `jsonwebtoken` crate to encode the given claims into a JWT
/// and signs it using the secret key defined in `JWT_SECRET`.
///
/// # Arguments
///
/// * `claims` - A `JwtClaims` struct containing the user ID and admin status to be encoded in the JWT.
///
/// # Returns
///
/// Returns a signed JWT as a `String`. If an error occurs during encoding, the function will panic.
///
/// # Example
///
/// ```
/// let claims = JwtClaims {
///     user_id: String::from("user123"),
///     is_admin: true,
/// };
/// let token = generate_jwt(claims);
/// println!("Generated JWT: {}", token);
/// ```
pub fn generate_jwt(claims: JwtClaims) -> String {
  encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(JWT_SECRET.as_ref()),
  ).unwrap() // Panics if the encoding fails. Handle errors properly in production.
}