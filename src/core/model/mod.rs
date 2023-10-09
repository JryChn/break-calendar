use chrono;
use chrono::{DateTime, Utc};

pub mod event;
pub mod reminder;
pub mod custom_info;


struct DurationTime {
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}

struct Location {
    name: String,
    lng: f32,
    lat: f32,
}

#[derive(Default)]
enum ImportanceLevel {
    VeryHigh,
    High,
    #[default]
    Medium,
    Low,
    VeryLow,
}

enum Time {
    Morning,
    Noon,
    AfterNoon,
    Night,
    Midnight,
    Weekday,
    Weekend,
    Holiday,
    NonHoliday,
    SleepTime,
    Other(u128, u128),
}
