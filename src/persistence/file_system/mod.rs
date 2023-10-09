use crate::common::exception::InternalError;
use crate::persistence::{CommonEvent, CustomInfo, DaoOperation};

pub struct FileSystemPersistence {
    file_name: String,
}

impl FileSystemPersistence {
    pub(crate) fn init() -> FileSystemPersistence {
        FileSystemPersistence {
            file_name: "metadata".into_string()
        }
    }
}

impl DaoOperation for FileSystemPersistence {
    fn save_mul_events(&self,event: Vec<CommonEvent>) -> Result<Vec<u128>, InternalError> {
        todo!()
    }

    fn get_all_events(&self) -> Option<Vec<CommonEvent>> {
        todo!()
    }

    fn save_custom_info(&self,custom_info: CustomInfo) -> Result<u128, InternalError> {
        todo!()
    }

    fn get_all_custom_info(&self,id: u128) -> Vec<CustomInfo> {
        todo!()
    }

    fn delete_custom_info(&self,id: u128) -> Result<Ok(()), InternalError> {
        todo!()
    }
}
