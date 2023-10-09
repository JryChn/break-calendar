use std::error::Error;
use crate::core::model::custom_info::CustomInfo;

pub fn create_custom_info(name:Option<String>) -> CustomInfo{
    todo!()
}

pub fn delete_custom_info(id : u128) -> Result<Ok(_),Err(dyn Error)>{
    todo!()
}


pub fn fetch_custom_info_by_id(id : u128) -> Option<CustomInfo>{
    todo!()
}

pub fn update_prefer_time(custom_info : CustomInfo){
    todo!()
}