use chrono::{DateTime, Utc, Duration};

const GIGASECOND: i64 = 1_000_000_000; 

pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(GIGASECOND);
}
