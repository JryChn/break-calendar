use std::fmt::Display;

use chrono::{DateTime, FixedOffset, Offset, TimeZone};
use downcast_rs::{Downcast, impl_downcast};
use serde::{Deserialize, Serialize};

use crate::common::utils::{MAX_EVENT_TIMESTAMP, MIN_EVENT_TIMESTAMP};
use crate::model::generator_instance::GeneratorInstance;
use crate::persistent::PersistentModel;

pub mod event;
pub mod generator_instance;
pub mod reminder;

pub trait EventCommonTrait: Downcast {
    fn get_id(&self) -> u128;
    fn get_kind(&self) -> Kind;
    fn get_title(&self) -> &str;
    fn set_title(&mut self, title: &str);
    fn get_description(&self) -> &str;
    fn set_description(&mut self, description: &str);
    fn get_start_time(&self) -> DateTime<FixedOffset>;
    fn get_end_time(&self) -> DateTime<FixedOffset>;
    fn set_duration(&mut self, start_time: DateTime<FixedOffset>, end_time: DateTime<FixedOffset>);
    fn get_color(&self) -> &str;
    fn set_color(&mut self, color: &str);
    fn set_importance(&mut self, important_level: ImportantLevel);
    fn get_importance(&self) -> ImportantLevel;
    fn get_categories(&self) -> Category;
    fn set_categories(&mut self, category: Category);
    fn set_generator_instance(&mut self, generator_instance_id: u128);
    fn get_generator_instance(&self) -> Option<u128>;
    fn convert_to(&self, generator_instance: Option<GeneratorInstance>) -> PersistentModel;

    fn check_valid(&self) -> bool {
        self.get_start_time().lt(&self.get_end_time())
            && self.get_start_time().naive_utc().ge(&MIN_EVENT_TIMESTAMP)
            && self.get_end_time().naive_utc().le(&MAX_EVENT_TIMESTAMP)
    }
}
impl_downcast!(EventCommonTrait);

pub enum ImportantLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

pub enum Category {
    Default,
    Other,
}

impl Display for ImportantLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ImportantLevel::Low => "Low".to_string(),
            ImportantLevel::Medium => "Medium".to_string(),
            ImportantLevel::High => "High".to_string(),
            ImportantLevel::VeryHigh => "VeryHigh".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<&str> for ImportantLevel {
    fn from(s: &str) -> Self {
        match s {
            "Low" => ImportantLevel::Low,
            "Medium" => ImportantLevel::Medium,
            "High" => ImportantLevel::High,
            "VeryHigh" => ImportantLevel::VeryHigh,
            _ => ImportantLevel::Medium,
        }
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Category::Default => "Default".to_string(),
            Category::Other => "Other".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<&str> for Category {
    fn from(s: &str) -> Self {
        match s {
            "Default" => Category::Default,
            "Other" => Category::Other,
            _ => Category::Default,
        }
    }
}

pub enum Kind {
    Event,
    Reminder,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Kind::Event => "Event".to_string(),
            Kind::Reminder => "Reminder".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<&str> for Kind {
    fn from(s: &str) -> Self {
        match s {
            "Event" => Kind::Event,
            "Reminder" => Kind::Reminder,
            _ => Kind::Event,
        }
    }
}
