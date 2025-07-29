use crate::components::{AcledEvent, EventCard};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EventsListProps {
    pub events: Vec<AcledEvent>,
    #[prop_or(false)]
    pub compact: bool,
    #[prop_or("Events".to_string())]
    pub title: String,
}

#[function_component(EventsList)]
pub fn events_list(props: &EventsListProps) -> Html {
    if props.events.is_empty() {
        return html! {
            <div class="no-data">
                {"No data loaded. Use the parameters on the left to fetch data."}
            </div>
        };
    }

    html! {
        <div class="events-list">
            <h4>{&props.title}</h4>
            {props.events.iter().map(|event| {
                html! {
                    <EventCard event={event.clone()} compact={props.compact} />
                }
            }).collect::<Html>()}
        </div>
    }
} 