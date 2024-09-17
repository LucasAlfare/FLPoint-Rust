use chrono::{DateTime, Local};

/// Represents a time-tracked point associated with a user.
///
/// This struct stores information about a specific time point related to a user,
/// typically used for tracking events such as timestamps for attendance or logging activities.
#[derive(Debug)]
pub struct Point {
  /// The unique identifier for the point.
  pub id: usize,

  /// The ID of the user related to this point.
  /// This links the point to a specific user in the system.
  pub related_user_id: usize,

  /// The exact date and time when the point was recorded.
  /// The time is captured using the local time zone.
  pub instant: DateTime<Local>,
}

impl Point {
  /// Creates a new `Point` instance with the provided details.
  ///
  /// # Arguments
  ///
  /// * `id` - The unique identifier for the point.
  /// * `related_user_id` - The ID of the user associated with this point.
  /// * `instant` - The date and time when this point was recorded.
  ///
  /// # Returns
  ///
  /// Returns a new `Point` instance initialized with the given data.
  ///
  /// # Example
  ///
  /// ```
  /// use chrono::Local;
  /// let now = Local::now();
  /// let point = Point::new(1, 42, now);
  /// ```
  pub fn new(id: usize, related_user_id: usize, instant: DateTime<Local>) -> Point {
    Point { id, related_user_id, instant }
  }
}