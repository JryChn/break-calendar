use crate::api::model::ApiEvent;
use crate::core::model::event::Event;
use crate::persistence::CommonEvent;

pub struct EventConvertor;

impl EventConvertor {
    pub fn convert_from_api_2_business(
        api_event: &ApiEvent, business_event: Event,
    ) -> Event {
        todo!()
    }
    pub fn convert_from_business_2_api(business_event: &Event, api_event: ApiEvent,
    ) -> ApiEvent {
        todo!()
    }
    pub fn convert_from_dao_2_business(common_event: &CommonEvent, business_event: Event,
    ) -> Event {
        todo!()
    }
    pub fn convert_from_business_2_dao(event: &Event, common_event: CommonEvent,
    ) -> CommonEvent {
        todo!()
    }
}