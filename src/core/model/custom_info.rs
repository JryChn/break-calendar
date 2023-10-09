use chrono::{DateTime, Utc};
use crate::core::model::Time;

pub struct CustomInfo {
    pub id: u128,
    pub name: String,
    pub portrait: Option<String>,
    pub birthday: Option<DateTime<Utc>>,
    pub gender: Option<Gender>,
    pub features: Vec<Feature>,
}


pub enum Gender {
    Female,
    Male,
}

pub enum Feature {
    PreferColor(String),
    PreferTime(Vec<Time>),
}
