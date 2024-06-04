use chrono::{DateTime, FixedOffset, Utc};
use uuid::Uuid;

use crate::model::{Category, EventCommonTrait, ImportantLevel, Kind};
use crate::model::generator_instance::GeneratorInstance;
use crate::persistent::PersistentModel;

pub struct Reminder {
    id: u128,
    title: String,
    description: String,
    start_time: DateTime<FixedOffset>,
    end_time: DateTime<FixedOffset>,
    color: String,
    important_level: String,
    category: String,
    generator_instance: Option<u128>,
}

impl EventCommonTrait for Reminder {
    fn get_id(&self) -> u128 {
        self.id
    }

    fn get_kind(&self) -> Kind {
        Kind::Reminder
    }

    fn get_title(&self) -> &str {
        self.title.as_str()
    }

    fn set_title(&mut self, title: &str) {
        self.title = title.to_string()
    }

    fn get_description(&self) -> &str {
        self.description.as_str()
    }

    fn set_description(&mut self, description: &str) {
        self.description = description.to_string()
    }

    fn get_start_time(&self) -> DateTime<FixedOffset> {
        self.start_time
    }

    fn get_end_time(&self) -> DateTime<FixedOffset> {
        self.end_time
    }

    fn set_duration(&mut self, start_time: DateTime<FixedOffset>, end_time: DateTime<FixedOffset>) {
        self.start_time = start_time;
        self.end_time = end_time;
    }

    fn get_color(&self) -> &str {
        self.color.as_str()
    }

    fn set_color(&mut self, color: &str) {
        self.color = color.to_string();
    }


    fn set_importance(&mut self, important_level: ImportantLevel) {
        self.important_level = important_level.to_string();
    }

    fn get_importance(&self) -> ImportantLevel {
        ImportantLevel::from(self.important_level.as_str())
    }

    fn get_categories(&self) -> Category {
        Category::from(self.category.as_str())
    }

    fn set_categories(&mut self, category: Category) {
        self.category = category.to_string();
    }

    fn set_generator_instance(&mut self, generator_instance: u128) {
        self.generator_instance = Some(generator_instance);
    }

    fn get_generator_instance(&self) -> Option<u128> {
        self.generator_instance
    }

    fn convert_to(&self, generator_instance: Option<GeneratorInstance>) -> PersistentModel {
        PersistentModel {
            id: self.id,
            kind: self.get_kind().to_string(),
            title: self.title.clone(),
            description: self.description.clone(),
            start_time: self.start_time.timestamp(),
            start_time_timezone: self.start_time.offset().to_string(),
            end_time: self.end_time.timestamp(),
            end_time_timezone: self.end_time.offset().to_string(),
            color: self.color.clone(),
            important_level: self.important_level.clone(),
            category: self.category.clone(),
            generator_instance,
        }
    }
}

impl Reminder {
    pub fn init(id: Option<u128>) -> Self {
        let uuid = Uuid::new_v4().as_u128();
        let now = DateTime::from(Utc::now());
        Reminder {
            id: if id.is_some() { id.unwrap() } else { uuid },
            title: "".to_string(),
            description: "".to_string(),
            start_time: now,
            end_time: now,
            color: "".to_string(),
            important_level: ImportantLevel::Low.to_string(),
            category: Category::Default.to_string(),
            generator_instance: None,
        }
    }
    pub fn self_clone(&self, is_new: bool) -> Self {
        Reminder {
            id: if is_new {
                Uuid::new_v4().as_u128()
            } else {
                self.id
            },
            title: self.title.clone(),
            description: self.description.clone(),
            start_time: self.start_time,
            end_time: self.end_time,
            color: self.color.clone(),
            important_level: self.important_level.clone(),
            category: self.category.clone(),
            generator_instance: self.generator_instance.clone(),
        }
    }
}
