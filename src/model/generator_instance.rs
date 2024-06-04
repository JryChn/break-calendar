use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneratorInstance {
    id: u128,
    pub repeat: Option<Repeat>,
    pub prefer_strategy: Option<PreferStrategy>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Repeat {
    pub repeat_strategy: RepeatStrategy,
    pub event_queue: Vec<u128>,
    pub throw_error_when_conflict: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RepeatStrategy {
    pub repeat_gap_day: u32,
    pub repeat_gap_month: u32,
    pub repeat_gap_year: u32,
    pub skip_weekday: bool,
    pub skip_weekend: bool,
    pub skip_holiday: bool,
    pub start_day: i64,
    pub until_day: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PreferStrategy {
    pub prefer_morning: bool,
    pub prefer_afternoon: bool,
    pub prefer_evening: bool,
    pub prefer_night: bool,
    pub prefer_weekday: bool,
    pub prefer_weekend: bool,
    pub prefer_holiday: bool,
    pub prefer_non_holiday: bool,
}

impl GeneratorInstance {
    pub fn new() -> Self {
        GeneratorInstance {
            id: Uuid::new_v4().as_u128(),
            repeat: None,
            prefer_strategy: None,
        }
    }
    pub fn get_id(&self) -> u128 {
        self.id
    }
}

impl RepeatStrategy {
    pub fn check_valid(repeat_strategy: &RepeatStrategy) -> bool {
        if repeat_strategy.until_day < repeat_strategy.start_day {
            return false;
        }
        if repeat_strategy.skip_weekday && repeat_strategy.skip_weekend {
            return false;
        }
        true
    }
}
