use crate::logging::console;
use crate::states::{AppState, UserData};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DashboardProps {
    pub user_data: UserData,
    pub on_state_change: Callback<AppState>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct AcledParams {
    pub start_date: String,
    pub end_date: String,
    pub country: String,
    pub event_type: String,
    pub limit: u32,
}

impl Default for AcledParams {
    fn default() -> Self {
        Self {
            start_date: "2024-01-01".to_string(),
            end_date: "2024-12-31".to_string(),
            country: "Lebanon".to_string(),
            event_type: "Battles".to_string(),
            limit: 100,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct AcledEvent {
    pub data_id: u32,
    pub event_date: String,
    pub event_type: String,
    pub actor1: String,
    pub actor2: String,
    pub location: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub notes: String,
    pub fatalities: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct AcledResponse {
    pub success: Option<bool>,
    pub count: Option<u32>,
    pub data: Option<Vec<AcledEvent>>,
    pub pagination: Option<serde_json::Value>,
    pub status: Option<u32>,
    pub error: Option<String>,
    pub message: Option<String>,
}

#[function_component(Dashboard)]
pub fn dashboard(props: &DashboardProps) -> Html {
    let params = use_state(|| AcledParams::default());
    let response = use_state(|| Option::<AcledResponse>::None);
    let loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    let show_user_info = use_state(|| false);

    let on_logout = {
        let on_state_change = props.on_state_change.clone();
        Callback::from(move |_: MouseEvent| {
            console::log_user_action!("Logout button clicked");

            if let Some(window) = web_sys::window() {
                console::log!("Clearing user credentials from storage");

                match window.local_storage() {
                    Ok(Some(storage)) => {
                        console::debug!("Local storage available for clearing");

                        match storage.remove_item("user_email") {
                            Ok(_) => console::log!("Successfully removed user email"),
                            Err(e) => console::error_with_context!(
                                "DASHBOARD",
                                "Error removing email: {:?}",
                                e
                            ),
                        }

                        match storage.remove_item("user_api_key") {
                            Ok(_) => console::log!("Successfully removed user API key"),
                            Err(e) => console::error_with_context!(
                                "DASHBOARD",
                                "Error removing API key: {:?}",
                                e
                            ),
                        }
                    }

                    Ok(None) => console::error_with_context!(
                        "DASHBOARD",
                        "Local storage not available for clearing"
                    ),
                    Err(e) => console::error_with_context!(
                        "DASHBOARD",
                        "Error accessing local storage for clearing: {:?}",
                        e
                    ),
                }
            } else {
                console::error_with_context!("DASHBOARD", "Window not available for clearing");
            }

            on_state_change.emit(AppState::Login);
            console::log_with_context!("DASHBOARD", "User state cleared");
        })
    };

    let toggle_user_info = {
        let show_user_info = show_user_info.clone();
        Callback::from(move |_: MouseEvent| {
            show_user_info.set(!*show_user_info);
        })
    };

    let close_user_info = {
        let show_user_info = show_user_info.clone();
        Callback::from(move |_: MouseEvent| {
            show_user_info.set(false);
        })
    };

    let fetch_acled_data = {
        let params = params.clone();
        let response = response.clone();
        let loading = loading.clone();
        let error = error.clone();
        let user_data = props.user_data.clone();

        Callback::from(move |_: MouseEvent| {
            let params = (*params).clone();
            let user_data = user_data.clone();
            let response = response.clone();
            let loading = loading.clone();
            let error = error.clone();

            loading.set(true);
            error.set(None);
            response.set(None);

            spawn_local(async move {
                let url = format!(
                    "https://api.acleddata.com/acled/read?email={}&key={}&format=json&limit={}&start_date={}&end_date={}&country={}&event_type={}",
                    user_data.email,
                    user_data.api_key,
                    params.limit,
                    params.start_date,
                    params.end_date,
                    params.country,
                    params.event_type
                );

                match Request::get(&url).send().await {
                    Ok(resp) => {
                        let status = resp.status();
                        match resp.text().await {
                            Ok(text) => {
                                console::log!("Raw API response: {}", text);

                                if status != 200 {
                                    error.set(Some(format!(
                                        "API returned status {}: {}",
                                        status, text
                                    )));
                                    loading.set(false);
                                    return;
                                }

                                match serde_json::from_str::<AcledResponse>(&text) {
                                    Ok(mut data) => {
                                        if let Some(err_msg) = &data.error {
                                            error.set(Some(format!("API Error: {}", err_msg)));
                                        } else if let Some(msg) = &data.message {
                                            error.set(Some(format!("API Message: {}", msg)));
                                        } else {
                                            if data.data.is_none() {
                                                match serde_json::from_str::<Vec<AcledEvent>>(&text)
                                                {
                                                    Ok(events) => {
                                                        data.data = Some(events);
                                                        data.count =
                                                            Some(data.data.as_ref().unwrap().len()
                                                                as u32);
                                                        data.success = Some(true);
                                                    }
                                                    Err(_) => {
                                                        error.set(Some(format!("Unexpected response format. Raw response: {}", text)));
                                                        loading.set(false);
                                                        return;
                                                    }
                                                }
                                            }
                                            response.set(Some(data));
                                        }
                                        loading.set(false);
                                    }
                                    Err(e) => {
                                        error.set(Some(format!(
                                            "Failed to parse JSON: {}. Raw response: {}",
                                            e, text
                                        )));
                                        loading.set(false);
                                    }
                                }
                            }
                            Err(e) => {
                                error.set(Some(format!("Failed to read response: {}", e)));
                                loading.set(false);
                            }
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Request failed: {}", e)));
                        loading.set(false);
                    }
                }
            });
        })
    };

    let update_param = {
        let params = params.clone();
        Callback::from(move |(field, value): (String, String)| {
            let mut new_params = (*params).clone();
            match field.as_str() {
                "start_date" => new_params.start_date = value,
                "end_date" => new_params.end_date = value,
                "country" => new_params.country = value,
                "event_type" => new_params.event_type = value,
                "limit" => {
                    if let Ok(limit) = value.parse::<u32>() {
                        new_params.limit = limit;
                    }
                }
                _ => {}
            }
            params.set(new_params);
        })
    };

    html! {
        <div class="dashboard">
            <div class="top-nav">
                <h1 class="dashboard-title">{"Dashboard"}</h1>
                <div class="user-nav">
                    <button class="nav-button user-info-button" onclick={toggle_user_info}>
                        {"User Info"}
                    </button>
                    <button class="nav-button logout-button" onclick={on_logout}>
                        {"Logout"}
                    </button>
                </div>
            </div>

            <div class="dashboard-layout">
                <div class="parameters-panel">
                    <h3>{"API Parameters"}</h3>

                    <div class="param-group">
                        <label>{"Start Date:"}</label>
                        <input
                            type="date"
                            value={params.start_date.clone()}
                            onchange={let update_param = update_param.clone();
                                Callback::from(move |e: Event| {
                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                    update_param.emit(("start_date".to_string(), target.value()));
                                })
                            }
                        />
                    </div>

                    <div class="param-group">
                        <label>{"End Date:"}</label>
                        <input
                            type="date"
                            value={params.end_date.clone()}
                            onchange={let update_param = update_param.clone();
                                Callback::from(move |e: Event| {
                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                    update_param.emit(("end_date".to_string(), target.value()));
                                })
                            }
                        />
                    </div>

                    <div class="param-group">
                        <label>{"Country:"}</label>
                        <input
                            type="text"
                            value={params.country.clone()}
                            placeholder="e.g., Syria, Iraq, Yemen"
                            onchange={let update_param = update_param.clone();
                                Callback::from(move |e: Event| {
                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                    update_param.emit(("country".to_string(), target.value()));
                                })
                            }
                        />
                    </div>

                    <div class="param-group">
                        <label>{"Event Type:"}</label>
                        <select
                            value={params.event_type.clone()}
                            onchange={let update_param = update_param.clone();
                                Callback::from(move |e: Event| {
                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                    update_param.emit(("event_type".to_string(), target.value()));
                                })
                            }
                        >
                            <option value="Battles">{"Battles"}</option>
                            <option value="Protests">{"Protests"}</option>
                            <option value="Riots">{"Riots"}</option>
                            <option value="Violence against civilians">{"Violence against civilians"}</option>
                            <option value="Explosions/Remote violence">{"Explosions/Remote violence"}</option>
                            <option value="Strategic developments">{"Strategic developments"}</option>
                        </select>
                    </div>

                    <div class="param-group">
                        <label>{"Limit:"}</label>
                        <input
                            type="number"
                            value={params.limit.to_string()}
                            min="1"
                            max="5000"
                            onchange={let update_param = update_param.clone();
                                Callback::from(move |e: Event| {
                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                    update_param.emit(("limit".to_string(), target.value()));
                                })
                            }
                        />
                    </div>

                    <button class="retro-button fetch-button" onclick={fetch_acled_data} disabled={*loading}>
                        if *loading {
                            {"Loading..."}
                        } else {
                            {"Fetch Data"}
                        }
                    </button>
                </div>

                <div class="response-panel">
                    <h3>{"API Response"}</h3>

                    if let Some(error_msg) = (*error).as_ref() {
                        <div class="error-message">
                            <strong>{"Error: "}</strong>{error_msg}
                        </div>
                    }

                    if let Some(acled_response) = (*response).as_ref() {
                        <div class="response-info">
                            <p><strong>{"Success: "}</strong>{if acled_response.success.unwrap_or(false) { "Yes" } else { "No" }}</p>
                            <p><strong>{"Total Records: "}</strong>{acled_response.count.unwrap_or(0)}</p>
                            <p><strong>{"Returned Records: "}</strong>{acled_response.data.as_ref().map_or(0, |d| d.len())}</p>
                        </div>

                        <div class="events-list">
                            <h4>{"Events"}</h4>
                            {acled_response.data.as_ref().map_or(Html::default(), |data| {
                                data.iter().map(|event| {
                                    html! {
                                        <div class="event-card">
                                            <div class="event-header">
                                                <span class="event-type">{&event.event_type}</span>
                                                <span class="event-date">{&event.event_date}</span>
                                            </div>
                                            <div class="event-location">
                                                <strong>{"Location: "}</strong>{&event.location}
                                            </div>
                                            <div class="event-actors">
                                                <div><strong>{"Actor 1: "}</strong>{&event.actor1}</div>
                                                <div><strong>{"Actor 2: "}</strong>{&event.actor2}</div>
                                            </div>
                                            if let Some(fatalities) = event.fatalities {
                                                <div class="event-fatalities">
                                                    <strong>{"Fatalities: "}</strong>{fatalities}
                                                </div>
                                            }
                                            <div class="event-notes">
                                                <strong>{"Notes: "}</strong>{&event.notes}
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()
                            })}
                        </div>
                    } else if *loading {
                        <div class="loading">
                            {"Loading ACLED data..."}
                        </div>
                    } else {
                        <div class="no-data">
                            {"No data loaded. Use the parameters on the left to fetch ACLED data."}
                        </div>
                    }
                </div>
            </div>

            if *show_user_info {
                <div class="modal-overlay" onclick={close_user_info.clone()}>
                    <div class="modal-content" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                        <div class="modal-header">
                            <h3>{"User Information"}</h3>
                            <button class="modal-close" onclick={close_user_info}>{"×"}</button>
                        </div>
                        <div class="modal-body">
                            <div class="user-info-item">
                                <strong>{"Email: "}</strong>
                                <span>{&props.user_data.email}</span>
                            </div>
                            <div class="user-info-item">
                                <strong>{"API Key: "}</strong>
                                <span>{"••••••••••••••••"}</span>
                            </div>
                            <div class="user-info-item">
                                <strong>{"Status: "}</strong>
                                <span class="status-active">{"Active"}</span>
                            </div>
                        </div>
                    </div>
                </div>
            }
        </div>
    }
}
