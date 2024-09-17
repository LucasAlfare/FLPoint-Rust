use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// Data Transfer Object (DTO) for creating a new point (timestamp) entry.
///
/// This struct encapsulates the data needed to create a point record,
/// which includes the related user's ID and the exact date and time of the point event.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePointRequestDTO {
  /// The unique identifier of the user related to this point.
  pub related_user_id: usize,

  /// The exact date and time when the point event occurred.
  /// The time is captured in the local time zone.
  pub instant: DateTime<Local>,
}

impl CreatePointRequestDTO {
  /// Creates a new `CreatePointRequestDTO` instance with the provided details.
  ///
  /// # Arguments
  ///
  /// * `related_user_id` - The unique ID of the user associated with this point event.
  /// * `instant` - The date and time when this point event occurred.
  ///
  /// # Returns
  ///
  /// Returns a new `CreatePointRequestDTO` initialized with the given information.
  ///
  /// # Example
  ///
  /// ```
  /// use chrono::Local;
  /// let now = Local::now();
  /// let point_dto = CreatePointRequestDTO::new(42, now);
  /// ```
  pub fn new(related_user_id: usize, instant: DateTime<Local>) -> Self {
    CreatePointRequestDTO { related_user_id, instant }
  }
}