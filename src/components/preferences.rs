use crate::states::UserData;
use crate::types::Theme;
use web_sys::MouseEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PreferencesProps {
    pub user_data: UserData,
    pub show: bool,
    pub current_theme: Theme,
    pub on_close: Callback<()>,
    pub on_theme_change: Callback<Theme>,
}

#[function_component(Preferences)]
pub fn preferences(props: &PreferencesProps) -> Html {
    let close_preferences = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    let select_light_theme = {
        let on_theme_change = props.on_theme_change.clone();
        Callback::from(move |_: MouseEvent| {
            on_theme_change.emit(Theme::Light);
        })
    };

    let select_blue_theme = {
        let on_theme_change = props.on_theme_change.clone();
        Callback::from(move |_: MouseEvent| {
            on_theme_change.emit(Theme::Blue);
        })
    };

    let select_dark_theme = {
        let on_theme_change = props.on_theme_change.clone();
        Callback::from(move |_: MouseEvent| {
            on_theme_change.emit(Theme::Dark);
        })
    };

    let select_terminal_theme = {
        let on_theme_change = props.on_theme_change.clone();
        Callback::from(move |_: MouseEvent| {
            on_theme_change.emit(Theme::Terminal);
        })
    };

    if props.show {
        html! {
            <div class="modal-overlay" onclick={close_preferences.clone()}>
                <div class="modal-content" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                    <div class="modal-header">
                        <h3>{"Preferences"}</h3>
                        <button class="modal-close" onclick={close_preferences}>{"×"}</button>
                    </div>
                    <div class="modal-body">
                        <div class="preferences-section">
                            <h4>{"Theme"}</h4>
                            <div class="theme-selector">
                                <button 
                                    class={if props.current_theme == Theme::Light { "theme-button active" } else { "theme-button" }}
                                    onclick={select_light_theme}
                                >
                                    {"Light"}
                                </button>
                                <button 
                                    class={if props.current_theme == Theme::Blue { "theme-button active" } else { "theme-button" }}
                                    onclick={select_blue_theme}
                                >
                                    {"Blue"}
                                </button>
                                <button 
                                    class={if props.current_theme == Theme::Dark { "theme-button active" } else { "theme-button" }}
                                    onclick={select_dark_theme}
                                >
                                    {"Dark"}
                                </button>
                                <button 
                                    class={if props.current_theme == Theme::Terminal { "theme-button active" } else { "theme-button" }}
                                    onclick={select_terminal_theme}
                                >
                                    {"Terminal"}
                                </button>
                            </div>
                        </div>
                        
                        <div class="preferences-section">
                            <h4>{"User Information"}</h4>
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
            </div>
        }
    } else {
        html! {}
    }
} 