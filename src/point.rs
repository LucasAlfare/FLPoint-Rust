use chrono::{DateTime, FixedOffset};

#[derive(Debug)]
pub struct Point {
  pub id: usize,
  pub related_user_id: usize,
  pub instant: DateTime<FixedOffset>,
}

impl Point {
  pub fn new(id: usize, related_user_id: usize, instant: DateTime<FixedOffset>) -> Point {
    Point { id, related_user_id, instant }
  }
}