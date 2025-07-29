use crate::config::{AVAILABLE_COUNTRIES, AVAILABLE_EVENT_TYPES, MAX_EVENTS_LIMIT};
use crate::errors::AppError;
use crate::logging::console;
use crate::services::AcledService;
use crate::types::{AcledEvent, AcledParams, UserData};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement, MouseEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MapDataProps {
    pub user_data: UserData,
    pub on_data_change: Callback<Option<Vec<AcledEvent>>>,
}

#[function_component(MapData)]
pub fn map_data(props: &MapDataProps) -> Html {
    let params = use_state(AcledParams::default);
    let loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);

    let update_param = {
        let params = params.clone();
        let error = error.clone();
        Callback::from(move |new_params: AcledParams| {
            params.set(new_params);
            error.set(None); // Clear error when params change
        })
    };

    let fetch_acled_data = {
        let user_data = props.user_data.clone();
        let params = params.clone();
        let loading = loading.clone();
        let error = error.clone();
        let on_data_change = props.on_data_change.clone();

        Callback::from(move |_: MouseEvent| {
            console::log_user_action!("Fetch button clicked");

            loading.set(true);
            error.set(None);

            let on_success = {
                let loading = loading.clone();
                let on_data_change = on_data_change.clone();
                Callback::from(move |events: Vec<AcledEvent>| {
                    console::log!("Successfully fetched {} events", events.len());
                    loading.set(false);
                    on_data_change.emit(Some(events));
                })
            };

            let on_error = {
                let loading = loading.clone();
                let error = error.clone();
                let on_data_change = on_data_change.clone();
                Callback::from(move |app_error: AppError| {
                    console::error_with_context!("API", "Failed to fetch data: {}", app_error);
                    loading.set(false);
                    error.set(Some(app_error.to_string()));
                    on_data_change.emit(None);
                })
            };

            AcledService::fetch_events(&user_data, &*params, on_success, on_error);
        })
    };

    html! {
                    <div class="panel parameters-panel">
            <h3>{"API Parameters"}</h3>

            <div class="param-group">
                <label for="start-date">{"Start Date:"}</label>
                <input
                    type="date"
                    id="start-date"
                    value={params.start_date.clone()}
                    onchange={
                        let update_param = update_param.clone();
                        let params = params.clone();
                        Callback::from(move |e: Event| {
                            let target = e.target().unwrap();
                            let input = target.unchecked_into::<HtmlInputElement>();
                            let mut new_params = (*params).clone();
                            new_params.start_date = input.value();
                            update_param.emit(new_params);
                        })
                    }
                />
            </div>

            <div class="param-group">
                <label for="end-date">{"End Date:"}</label>
                <input
                    type="date"
                    id="end-date"
                    value={params.end_date.clone()}
                    onchange={
                        let update_param = update_param.clone();
                        let params = params.clone();
                        Callback::from(move |e: Event| {
                            let target = e.target().unwrap();
                            let input = target.unchecked_into::<HtmlInputElement>();
                            let mut new_params = (*params).clone();
                            new_params.end_date = input.value();
                            update_param.emit(new_params);
                        })
                    }
                />
            </div>

            <div class="param-group">
                <label for="country">{"Country:"}</label>
                <select
                    id="country"
                    value={params.country.clone()}
                    onchange={
                        let update_param = update_param.clone();
                        let params = params.clone();
                        Callback::from(move |e: Event| {
                            let target = e.target().unwrap();
                            let select = target.unchecked_into::<HtmlSelectElement>();
                            let mut new_params = (*params).clone();
                            new_params.country = select.value();
                            update_param.emit(new_params);
                        })
                    }
                >
                    {for AVAILABLE_COUNTRIES.iter().map(|&country| {
                        html! {
                            <option value={country} selected={params.country == country}>
                                {country}
                            </option>
                        }
                    })}
                </select>
            </div>

            <div class="param-group">
                <label for="event-type">{"Event Type:"}</label>
                <select
                    id="event-type"
                    value={params.event_type.clone()}
                    onchange={
                        let update_param = update_param.clone();
                        let params = params.clone();
                        Callback::from(move |e: Event| {
                            let target = e.target().unwrap();
                            let select = target.unchecked_into::<HtmlSelectElement>();
                            let mut new_params = (*params).clone();
                            new_params.event_type = select.value();
                            update_param.emit(new_params);
                        })
                    }
                >
                    {for AVAILABLE_EVENT_TYPES.iter().map(|&event_type| {
                        html! {
                            <option value={event_type} selected={params.event_type == event_type}>
                                {event_type}
                            </option>
                        }
                    })}
                </select>
            </div>

            <div class="param-group">
                <label for="limit">{"Limit:"}</label>
                <input
                    type="number"
                    id="limit"
                    min="1"
                    max={MAX_EVENTS_LIMIT.to_string()}
                    value={params.limit.to_string()}
                    onchange={
                        let update_param = update_param.clone();
                        let params = params.clone();
                        Callback::from(move |e: Event| {
                            let target = e.target().unwrap();
                            let input = target.unchecked_into::<HtmlInputElement>();
                            if let Ok(limit) = input.value().parse::<u32>() {
                                let mut new_params = (*params).clone();
                                new_params.limit = limit;
                                update_param.emit(new_params);
                            }
                        })
                    }
                />
            </div>

            <button
                class="button"
                onclick={fetch_acled_data}
                disabled={*loading}
            >
                {if *loading {
                    "Loading..."
                } else {
                    "Fetch Data"
                }}
            </button>

            {if let Some(error_msg) = (*error).as_ref() {
                html! {
                    <div class="error-message">
                        <strong>{"Error: "}</strong>{error_msg}
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
