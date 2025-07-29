use crate::components::{AcledEvent, MapData, MapDisplay, ResponseDisplay};
use crate::types::{AppState, DashboardView, UserData};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DashboardProps {
    pub user_data: UserData,
    pub current_view: DashboardView,
    pub on_state_change: Callback<AppState>,
}

#[function_component(Dashboard)]
pub fn dashboard(props: &DashboardProps) -> Html {
    let events_data = use_state(|| Option::<Vec<AcledEvent>>::None);

    let on_data_change = {
        let events_data = events_data.clone();
        Callback::from(move |new_events: Option<Vec<AcledEvent>>| {
            events_data.set(new_events);
        })
    };

    html! {
        <div class="dashboard">
            <div class="dashboard-layout">
                <MapData user_data={props.user_data.clone()} on_data_change={on_data_change} />

                <div class="panel response-panel">
                    <div class="view-content">
                        {match props.current_view {
                            DashboardView::Map => html! {
                                <MapDisplay events={(*events_data).clone()} />
                            },
                            DashboardView::DataList => html! {
                                <ResponseDisplay events={(*events_data).clone()} />
                            },
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
