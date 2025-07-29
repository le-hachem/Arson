use crate::components::{AcledEvent, EventsList, ResponseInfo};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ResponseDisplayProps {
    pub events: Option<Vec<AcledEvent>>,
    #[prop_or(true)]
    pub show_response_info: bool,
}

#[function_component(ResponseDisplay)]
pub fn response_display(props: &ResponseDisplayProps) -> Html {
    html! {
        <div class="response-content">
            if props.show_response_info {
                if let Some(events) = &props.events {
                    <ResponseInfo
                        total_records={events.len() as u32}
                        returned_records={events.len() as u32}
                        success={true}
                    />
                }
            }

            if let Some(events) = &props.events {
                <EventsList events={events.clone()} title="Events" />
            } else {
                <div class="no-data">
                    {"No data loaded. Use the parameters on the left to fetch data."}
                </div>
            }
        </div>
    }
}
