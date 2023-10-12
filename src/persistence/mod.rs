use crate::common::exception::InternalError;

pub mod file_system;

pub trait DaoOperation {
    fn save_mul_events(&self,event: Vec<CommonEvent>) -> Result<Vec<u128>, InternalError>;
    fn get_all_events(&self) -> Option<Vec<CommonEvent>>;
    fn save_custom_info(&self,custom_info: CustomInfo) -> Result<u128, InternalError>;

    fn get_all_custom_info(&self,id: u128) -> Vec<CustomInfo>;

    fn delete_custom_info(&self,id: u128) -> Result<Ok(()), InternalError>;
}

pub struct CommonEvent {
    pub id: u128,
    pub series_id: Option<u128>,
    pub author: Option<u128>,
    pub title: String,
    pub description: String,
    pub location: Option<String>,
    pub people: Vec<String>,
    pub category: Option<String>,
    pub importance: String,
    pub start_time: i64,
    pub end_time: i64,
    pub tags: Vec<String>,
}
pub struct CustomInfo {
    pub id: u128,
    pub name: String,
    pub portrait: Option<String>,
    pub gender: u8,
    pub birthday: i64,
    pub tags: Vec<String>,
}
