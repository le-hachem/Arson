use crate::logging::console;
use crate::states::{AppState, UserData};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DashboardProps {
    pub user_data: UserData,
    pub on_state_change: Callback<AppState>,
}

#[function_component(Dashboard)]
pub fn dashboard(props: &DashboardProps) -> Html {
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

    html! {
        <div class="dashboard">
            <h1 id="title">{"Dashboard"}</h1>
            <p class="description">
                {"Welcome back! Your data is loaded."}
            </p>

            <div class="user-info">
                <h3>{"User Information"}</h3>
                <p><strong>{"Email: "}</strong> {&props.user_data.email}</p>
                <p><strong>{"API Key: "}</strong> {"••••••••••••••••"}</p>
            </div>

            <button class="retro-button logout-button" onclick={on_logout}>
                {"Logout"}
            </button>
        </div>
    }
}
