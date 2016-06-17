extern crate chrono;
use chrono::*;

pub fn after(date: datetime::DateTime<UTC>) -> datetime::DateTime<UTC> {
    date + Duration::seconds(10i64.pow(9))
}
