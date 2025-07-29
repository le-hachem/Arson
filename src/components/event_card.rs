use crate::components::AcledEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EventCardProps {
    pub event: AcledEvent,
    #[prop_or(false)]
    pub compact: bool,
}

#[function_component(EventCard)]
pub fn event_card(props: &EventCardProps) -> Html {
    let notes = if props.compact && props.event.notes.len() > 100 {
        format!("{}...", &props.event.notes[..100])
    } else {
        props.event.notes.clone()
    };

    html! {
        <div class="event-card">
            <div class="event-header">
                <span class="event-type">{&props.event.event_type}</span>
                <span class="event-date">{&props.event.event_date}</span>
            </div>
            <div class="event-location">
                <strong>{"Location: "}</strong>{&props.event.location}
            </div>
            <div class="event-actors">
                <div><strong>{"Actor 1: "}</strong>{&props.event.actor1}</div>
                <div><strong>{"Actor 2: "}</strong>{&props.event.actor2}</div>
            </div>
            if let Some(fatalities) = props.event.fatalities {
                <div class="event-fatalities">
                    <strong>{"Fatalities: "}</strong>{fatalities}
                </div>
            }
            <div class="event-notes">
                <strong>{"Notes: "}</strong>{notes}
            </div>
        </div>
    }
}

// Function to generate HTML popup content for Leaflet markers
pub fn generate_popup_content(event: &AcledEvent) -> String {
    let fatalities_html = event.fatalities.map_or(String::new(), |f| {
        format!(
            "<p style='margin: 4px 0; color: #ff6666;'><strong>Fatalities:</strong> {}</p>",
            f
        )
    });

    let notes = if event.notes.len() > 100 {
        format!("{}...", &event.notes[..100])
    } else {
        event.notes.clone()
    };

    format!(
        "<div style='color: #00ff00; background: #0a0a0a; font-family: \"Pixelify Sans\";'>
            <h4 style='margin: 0 0 8px 0; color: #00ff00;'>{}</h4>
            <p style='margin: 4px 0;'><strong>Date:</strong> {}</p>
            <p style='margin: 4px 0;'><strong>Location:</strong> {}</p>
            <p style='margin: 4px 0;'><strong>Actor 1:</strong> {}</p>
            <p style='margin: 4px 0;'><strong>Actor 2:</strong> {}</p>
            {}
            <p style='margin: 4px 0; font-style: italic; font-size: 0.9em;'>{}</p>
        </div>",
        event.event_type,
        event.event_date,
        event.location,
        event.actor1,
        event.actor2,
        fatalities_html,
        notes
    )
}
