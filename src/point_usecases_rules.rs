use chrono::{DateTime, Local, TimeDelta};

pub fn is_within_time_range(check: DateTime<Local>) -> bool {
  let now = Local::now();
  let lower_bound = now - TimeDelta::seconds(10);
  let higher_bound = now - TimeDelta::seconds(1);
  check >= lower_bound && check <= higher_bound
}

pub fn passed_at_least_30_min_from_last(last: DateTime<Local>, check: DateTime<Local>) -> bool {
  check - last >= TimeDelta::minutes(30)
}