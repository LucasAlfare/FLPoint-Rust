use chrono::{DateTime, Local, TimeDelta};

/// Checks if a given `DateTime` falls within a specified time range relative to the current time.
///
/// The time range is defined as being between 10 seconds in the past and 1 second in the future from the current time.
///
/// # Arguments
///
/// * `check` - The `DateTime` to be checked against the current time.
///
/// # Returns
///
/// Returns `true` if the `check` time is within the range of 10 seconds before and 1 second after the current time.
/// Otherwise, returns `false`.
///
/// # Example
///
/// ```
/// use chrono::Local;
/// let check_time = Local::now() - chrono::Duration::seconds(5);
/// let is_within_range = is_within_time_range(check_time);
/// println!("Is within range: {}", is_within_range);
/// ```
pub fn is_within_time_range(check: DateTime<Local>) -> bool {
  let now = Local::now();
  let lower_bound = now - TimeDelta::seconds(10);
  let higher_bound = now + TimeDelta::seconds(1);
  check >= lower_bound && check <= higher_bound
}

/// Checks if at least 30 minutes have passed between two `DateTime` instances.
///
/// This function compares a `last` recorded time and a `check` time to determine if at least 30 minutes
/// have passed since the `last` time.
///
/// # Arguments
///
/// * `last` - The `DateTime` representing the last recorded time.
/// * `check` - The `DateTime` to compare against the `last` time.
///
/// # Returns
///
/// Returns `true` if 30 minutes or more have passed between `last` and `check`. Otherwise, returns `false`.
///
/// # Example
///
/// ```
/// use chrono::Local;
/// let last_time = Local::now() - chrono::Duration::minutes(31);
/// let check_time = Local::now();
/// let is_past_30_min = passed_at_least_30_min_from_last(last_time, check_time);
/// println!("Has it been at least 30 minutes? {}", is_past_30_min);
/// ```
pub fn passed_at_least_30_min_from_last(last: DateTime<Local>, check: DateTime<Local>) -> bool {
  check - last >= TimeDelta::minutes(30)
}