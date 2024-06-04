use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::cache::Cache;
use crate::common::utils::convert_from_string_to_datetime;
use crate::model::{Category, EventCommonTrait, ImportantLevel, Kind};
use crate::model::event::Event;
use crate::model::generator_instance::GeneratorInstance;
use crate::model::reminder::Reminder;
use crate::persistent::file_system::FilePersistenceSystem;

mod file_system;

pub struct Persistent;

impl Persistent {
    pub fn init() -> Self {
        Persistent
    }
    pub fn save(&self, cache: &Cache) -> Result<()> {
        FilePersistenceSystem::save(cache, None)
    }
    pub fn load(&self) -> Result<Cache> {
        FilePersistenceSystem::load(None)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentModel {
    pub id: u128,
    pub kind: String,
    pub title: String,
    pub description: String,
    pub start_time: i64,
    pub start_time_timezone: String,
    pub end_time: i64,
    pub end_time_timezone: String,
    pub color: String,
    pub important_level: String,
    pub category: String,
    pub generator_instance: Option<GeneratorInstance>,
}

impl PersistentModel {
    pub fn convert_to(&self) -> Box<dyn EventCommonTrait> {
        match Kind::from(self.kind.clone().as_str()) {
            Kind::Event => {
                let mut event = Event::init(Some(self.id));
                event.set_title(self.title.as_str());
                event.set_description(self.description.as_str());
                event.set_duration(
                    convert_from_string_to_datetime(
                        self.start_time,
                        self.start_time_timezone.clone(),
                    ),
                    convert_from_string_to_datetime(self.end_time, self.end_time_timezone.clone()),
                );
                event.set_color(self.color.as_str());
                event.set_importance(ImportantLevel::from(self.important_level.clone().as_str()));
                event.set_categories(Category::from(self.category.clone().as_str()));
                if self.generator_instance.is_some() {
                    event.set_generator_instance(self.generator_instance.clone().unwrap().get_id());
                }
                Box::new(event)
            }
            Kind::Reminder => {
                let mut reminder = Reminder::init(Some(self.id));
                reminder.set_title(self.title.as_str());
                reminder.set_description(self.description.as_str());
                reminder.set_duration(
                    convert_from_string_to_datetime(
                        self.start_time,
                        self.start_time_timezone.clone(),
                    ),
                    convert_from_string_to_datetime(self.end_time, self.end_time_timezone.clone()),
                );
                reminder.set_color(self.color.as_str());
                reminder
                    .set_importance(ImportantLevel::from(self.important_level.clone().as_str()));
                reminder.set_categories(Category::from(self.category.clone().as_str()));
                if self.generator_instance.is_some() {
                    reminder
                        .set_generator_instance(self.generator_instance.clone().unwrap().get_id());
                }
                Box::new(reminder)
            }
        }
    }
}
