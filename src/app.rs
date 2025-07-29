use crate::components::Titlebar;
use crate::logging::console;
use crate::states::{
    dataview::DataView, login::Login, map::Map, AppState, DashboardView, UserData,
};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let app_state = use_state(|| AppState::Login);
    let storage_checked = use_state(|| false);

    {
        let app_state = app_state.clone();
        let storage_checked = storage_checked.clone();
        use_effect(move || {
            if !*storage_checked {
                storage_checked.set(true);

                if let Some(window) = web_sys::window() {
                    console::log!("Loading user credentials from storage");

                    match window.local_storage() {
                        Ok(Some(storage)) => {
                            console::debug!("Local storage available");

                            match storage.get_item("user_email") {
                                Ok(Some(saved_email)) => {
                                    console::debug!("Found saved email: {}", saved_email);

                                    match storage.get_item("user_api_key") {
                                        Ok(Some(saved_api_key)) => {
                                            console::debug!("Found saved API key");

                                            if !saved_email.is_empty() && !saved_api_key.is_empty()
                                            {
                                                console::log_with_context!(
                                                    "APP",
                                                    "Loading saved user data"
                                                );
                                                app_state.set(AppState::Dashboard(
                                                    UserData {
                                                        email: saved_email,
                                                        api_key: saved_api_key,
                                                    },
                                                    DashboardView::Data,
                                                ));
                                            } else {
                                                console::warn!("Saved data is empty");
                                            }
                                        }
                                        Ok(None) => console::debug!("No saved API key found"),
                                        Err(e) => console::error_with_context!(
                                            "APP",
                                            "Error getting API key: {:?}",
                                            e
                                        ),
                                    }
                                }
                                Ok(None) => console::debug!("No saved email found"),
                                Err(e) => console::error_with_context!(
                                    "APP",
                                    "Error getting email: {:?}",
                                    e
                                ),
                            }
                        }
                        Ok(None) => {
                            console::error_with_context!("APP", "Local storage not available")
                        }
                        Err(e) => console::error_with_context!(
                            "APP",
                            "Error accessing local storage: {:?}",
                            e
                        ),
                    }
                } else {
                    console::error_with_context!("APP", "Window not available");
                }
            }
            || ()
        });
    }

    let on_state_change = {
        let app_state = app_state.clone();
        Callback::from(move |new_state: AppState| {
            app_state.set(new_state);
        })
    };

    let on_view_change = {
        let app_state = app_state.clone();
        Callback::from(move |new_view: DashboardView| {
            if let AppState::Dashboard(user_data, _) = (*app_state).clone() {
                app_state.set(AppState::Dashboard(user_data, new_view));
            }
        })
    };

    html! {
        <div class="app">
            <Titlebar
                app_state={(*app_state).clone()}
                on_state_change={on_state_change.clone()}
                on_view_change={Some(on_view_change)}
            />
            <main class="container">
                {
                    match (*app_state).clone() {
                        AppState::Login => html! {
                            <Login on_state_change={on_state_change} />
                        },
                        AppState::Dashboard(user_data, view) => {
                            html! {
                                {match view {
                                    DashboardView::Data => html! {
                                        <DataView user_data={user_data.clone()} />
                                    },
                                    DashboardView::Map => html! {
                                        <Map user_data={user_data.clone()} />
                                    }
                                }}
                            }
                        }
                    }
                }
            </main>
        </div>
    }
}
