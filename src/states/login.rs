use crate::logging::console;
use crate::states::{AppState, DashboardView, UserData};
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LoginProps {
    pub on_state_change: Callback<AppState>,
}

#[function_component(Login)]
pub fn login(props: &LoginProps) -> Html {
    let email = use_state(String::new);
    let api_key = use_state(String::new);

    let on_email_input = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let target = e
                .target()
                .unwrap()
                .unchecked_into::<web_sys::HtmlInputElement>();
            email.set(target.value());
        })
    };

    let on_api_key_input = {
        let api_key = api_key.clone();
        Callback::from(move |e: InputEvent| {
            let target = e
                .target()
                .unwrap()
                .unchecked_into::<web_sys::HtmlInputElement>();
            api_key.set(target.value());
        })
    };

    let on_submit = {
        let email = email.clone();
        let api_key = api_key.clone();
        let on_state_change = props.on_state_change.clone();
        Callback::from(move |_: MouseEvent| {
            let email_value = (*email).clone();
            let api_key_value = (*api_key).clone();

            console::log_user_action!("Submit button clicked");

            if !email_value.is_empty() && !api_key_value.is_empty() {
                console::log_with_context!("LOGIN", "Form data is valid, updating state");
                let user_data = UserData {
                    email: email_value.clone(),
                    api_key: api_key_value.clone(),
                };
                on_state_change.emit(AppState::Dashboard(user_data, DashboardView::Data));

                if let Some(window) = web_sys::window() {
                    console::log!("Saving user credentials to storage");

                    match window.local_storage() {
                        Ok(Some(storage)) => {
                            console::debug!("Local storage available for saving");

                            match storage.set_item("user_email", &email_value) {
                                Ok(_) => {
                                    console::log!("Successfully saved user email");
                                }
                                Err(e) => {
                                    console::error_with_context!(
                                        "LOGIN",
                                        "Error saving email: {:?}",
                                        e
                                    )
                                }
                            }

                            match storage.set_item("user_api_key", &api_key_value) {
                                Ok(_) => {
                                    console::log!("Successfully saved user API key");
                                }
                                Err(e) => console::error_with_context!(
                                    "LOGIN",
                                    "Error saving API key: {:?}",
                                    e
                                ),
                            }
                        }
                        Ok(None) => console::error_with_context!(
                            "LOGIN",
                            "Local storage not available for saving"
                        ),
                        Err(e) => console::error_with_context!(
                            "LOGIN",
                            "Error accessing local storage for saving: {:?}",
                            e
                        ),
                    }
                } else {
                    console::error_with_context!("LOGIN", "Window not available for saving");
                }
            } else {
                console::warn!("Form data is invalid - email or API key is empty");
            }
        })
    };

    html! {
        <>
            <h1 id="title">{"Arson"}</h1>

            <div class="form-container">
                <div class="input-group">
                    <label for="email">{"Email Address:"}</label>
                    <input
                        type="email"
                        id="email"
                        class="input"
                        placeholder="Enter your email"
                        value={(*email).clone()}
                        oninput={on_email_input}
                    />
                </div>

                <div class="input-group">
                    <label for="api-key">{"ACLED API Key:"}</label>
                    <input
                        type="password"
                        id="api-key"
                        class="input"
                        placeholder="Enter your ACLED API key"
                        value={(*api_key).clone()}
                        oninput={on_api_key_input}
                    />
                </div>

                <button class="button" onclick={on_submit}>{"Initialize"}</button>
            </div>
        </>
    }
}
