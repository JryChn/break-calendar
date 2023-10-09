use crate::api::model::ApiReminder;
use crate::core::model::reminder::Reminder;
use crate::persistence::{CommonEvent, DaoOperation};

pub struct ReminderConvertor {
    dao_operator: dyn DaoOperation,
}

impl ReminderConvertor {
    pub fn new(dao_operator: impl DaoOperation) -> Box<ReminderConvertor> {
        Box::new(
            ReminderConvertor {
                dao_operator
            })
    }


    pub fn convert_from_api_2_business(&self,
        api_reminder: ApiReminder
    ) -> Reminder {
        todo!()
    }
    pub fn convert_from_business_2_api(&self,
        reminder: Reminder
    ) -> ApiReminder {
        todo!()
    }
    pub
    fn convert_from_dao_2_business(&self,
        reminder: CommonEvent) -> Reminder {
        todo!()
    }
    pub fn convert_from_business_2_dao(&self,
        reminder: Reminder
    ) -> CommonEvent {
        todo!()
    }
}
