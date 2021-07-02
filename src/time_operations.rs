use chrono::prelude::*;

pub fn get_date_time() -> i64 {
    Utc::now().timestamp_millis()
}