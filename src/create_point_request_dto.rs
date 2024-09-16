use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePointRequestDTO {
  pub related_user_id: usize,
  pub instant: DateTime<FixedOffset>,
}

impl CreatePointRequestDTO {
  pub fn new(related_user_id: usize, instant: DateTime<FixedOffset>) -> Self {
    CreatePointRequestDTO { related_user_id, instant }
  }
}