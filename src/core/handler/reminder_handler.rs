use std::error::Error;
use crate::core::model::reminder::Reminder;

pub fn create_reminder() ->Reminder{
    todo!()
}


pub fn get_reminder(id:u128) -> Option<Reminder>{
    todo!()
}

pub fn delete_reminder(id:u128) -> Result<Ok(_),Err(dyn Error)>{
    todo!()
}

pub fn fetch_all_undone_reminders()-> Vec<Reminder>{
    todo!()
}