use crate::logging::console;
use crate::services::StorageService;
use crate::types::{AppState, UserData};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LoginProps {
    pub on_state_change: Callback<AppState>,
}

#[function_component(Login)]
pub fn login(props: &LoginProps) -> Html {
    let email = use_state(|| String::new());
    let api_key = use_state(|| String::new());
    let error_message = use_state(|| Option::<String>::None);

    // Try to load saved credentials on component mount
    {
        let email = email.clone();
        let api_key = api_key.clone();
        use_effect_with((), move |_| {
            if let Ok(Some(user_data)) = StorageService::load_user_data() {
                email.set(user_data.email);
                api_key.set(user_data.api_key);
            }
            || {}
        });
    }

    let on_email_input = {
        let email = email.clone();
        let error_message = error_message.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap();
            let input = target.unchecked_into::<HtmlInputElement>();
            email.set(input.value());
            error_message.set(None); // Clear error when user types
        })
    };

    let on_api_key_input = {
        let api_key = api_key.clone();
        let error_message = error_message.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap();
            let input = target.unchecked_into::<HtmlInputElement>();
            api_key.set(input.value());
            error_message.set(None); // Clear error when user types
        })
    };

    let on_submit = {
        let on_state_change = props.on_state_change.clone();
        let email = email.clone();
        let api_key = api_key.clone();
        let error_message = error_message.clone();

        Callback::from(move |_: MouseEvent| {
            console::log_user_action!("Submit button clicked");
            let email_value = (*email).clone();
            let api_key_value = (*api_key).clone();

            console::log!(
                "Form submitted with email: {} and API key length: {}",
                email_value,
                api_key_value.len()
            );

            // Create and validate user data
            let user_data = UserData::new(email_value, api_key_value);

            if !user_data.is_valid() {
                error_message.set(Some("Please provide both email and API key.".to_string()));
                console::warn!("Form validation failed - missing email or API key");
                return;
            }

            console::log!("Creating user data");

            // Try to save user data
            if let Err(e) = StorageService::save_user_data(&user_data) {
                console::error_with_context!("LOGIN", "Failed to save credentials: {}", e);
                // Continue anyway, don't block login
            } else {
                console::log_with_context!("LOGIN", "User credentials saved successfully");
            }

            // Transition to dashboard
            on_state_change.emit(AppState::Dashboard(user_data));
        })
    };

    html! {
        <>
            <h1 id="title">{"Arson"}</h1>

            <div class="form-container">
                {if let Some(error) = (*error_message).as_ref() {
                    html! {
                        <div class="error-message" style="margin-bottom: 1rem;">
                            <strong>{"Error: "}</strong>{error}
                        </div>
                    }
                } else {
                    html! {}
                }}

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
