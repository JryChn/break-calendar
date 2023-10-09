use crate::api::model::ApiCustomInformation;
use crate::core::model::custom_info::CustomInfo;
use crate::persistence::DaoOperation;

struct CustomInfoConvertor {
    dao_operator: dyn DaoOperation,
}


impl CustomInfoConvertor {
    pub fn new(dao_operator: impl DaoOperation) -> Box<CustomInfoConvertor> {
        Box::new(
            CustomInfoConvertor {
                dao_operator
            })
    }
    pub fn convert_from_api_2_business(&self,
        custom_information: ApiCustomInformation) -> CustomInfo {
        todo!()
    }
    pub fn convert_from_business_2_api(&self,custom_info: CustomInfo) -> ApiCustomInformation {
        todo!()
    }
    pub fn convert_from_dao_2_business(&self,custom_info: crate::persistence::CustomInfo) -> CustomInfo {
        todo!()
    }
    pub fn convert_from_business_2_dao(&self,
        custom_info: CustomInfo
    ) -> crate::persistence::CustomInfo {
        todo!()
    }
}