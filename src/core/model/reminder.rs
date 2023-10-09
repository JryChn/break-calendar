use chrono::{DateTime, Utc};

pub struct Reminder {
    id: u128,
    title: String,
    remind_time: DateTime<Utc>,
    pub status: Vec<ReminderStatus>,
}
enum ReminderStatus {
    IsAllDayReminder(bool),
    IsDone(bool),
}
