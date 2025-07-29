use crate::components::Preferences;
use crate::services::StorageService;
use crate::types::{AppState, DashboardView, Theme};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Properties, PartialEq)]
pub struct TitlebarProps {
    pub app_state: AppState,
    pub dashboard_view: DashboardView,
    pub current_theme: Theme,
    pub on_state_change: Callback<AppState>,
    pub on_view_change: Callback<DashboardView>,
    pub on_theme_change: Callback<Theme>,
}

#[function_component(Titlebar)]
pub fn titlebar(props: &TitlebarProps) -> Html {
    let show_preferences = use_state(|| false);

    let minimize_onclick = Callback::from(|_| {
        spawn_local(async move {
            let _ = invoke("minimize_window", JsValue::NULL).await;
        });
    });

    let maximize_onclick = Callback::from(|_| {
        spawn_local(async move {
            let _ = invoke("maximize_window", JsValue::NULL).await;
        });
    });

    let close_onclick = Callback::from(|_| {
        spawn_local(async move {
            let _ = invoke("close_window", JsValue::NULL).await;
        });
    });

    let on_logout = {
        let on_state_change = props.on_state_change.clone();
        Callback::from(move |_: MouseEvent| {
            // Use the service for storage management
            if let Err(e) = StorageService::clear_user_data() {
                console_error!("Failed to clear user data: {}", e);
            }
            on_state_change.emit(AppState::Login);
        })
    };

    let toggle_preferences = {
        let show_preferences = show_preferences.clone();
        Callback::from(move |_: MouseEvent| {
            show_preferences.set(!*show_preferences);
        })
    };

    let close_preferences = {
        let show_preferences = show_preferences.clone();
        Callback::from(move |_| {
            show_preferences.set(false);
        })
    };

    let switch_to_map = {
        let on_view_change = props.on_view_change.clone();
        Callback::from(move |_: MouseEvent| {
            on_view_change.emit(DashboardView::Map);
        })
    };

    let switch_to_data = {
        let on_view_change = props.on_view_change.clone();
        Callback::from(move |_: MouseEvent| {
            on_view_change.emit(DashboardView::DataList);
        })
    };

    html! {
        <>
            <div class="titlebar">
                <div class="titlebar-left">
                    {match &props.app_state {
                        AppState::Dashboard(_) => html! {
                            <div class="titlebar-tabs">
                                <button
                                    class={if props.dashboard_view == DashboardView::DataList { "tab-button active" } else { "tab-button" }}
                                    onclick={switch_to_data}
                                >
                                    {"Data View"}
                                </button>
                                <button
                                    class={if props.dashboard_view == DashboardView::Map { "tab-button active" } else { "tab-button" }}
                                    onclick={switch_to_map}
                                >
                                    {"Map View"}
                                </button>
                            </div>
                        },
                        AppState::Login => html! {},
                    }}
                </div>

                <div class="titlebar-center">
                    {match &props.app_state {
                        AppState::Login => html! {
                            <span class="page-title">{""}</span>
                        },
                        AppState::Dashboard(_) => html! {
                            <span class="page-title">
                                {match props.dashboard_view {
                                    DashboardView::Map => "Interactive Map",
                                    DashboardView::DataList => "Dashboard",
                                }}
                            </span>
                        },
                    }}
                </div>

                <div class="titlebar-right">
                    {match &props.app_state {
                        AppState::Dashboard(_) => html! {
                            <>
                                <button class="titlebar-nav-button preferences-button" onclick={toggle_preferences}>
                                    {"Preferences"}
                                </button>
                                <button class="titlebar-nav-button logout-button" onclick={on_logout}>
                                    {"Logout"}
                                </button>
                            </>
                        },
                        AppState::Login => html! {},
                    }}

                    <div class="titlebar-controls">
                        <button class="titlebar-button minimize" onclick={minimize_onclick}>
                            <svg viewBox="0 0 6 6">
                                <rect x="0" y="2" width="1" height="1" fill="currentColor"/>
                                <rect x="1" y="2" width="1" height="1" fill="currentColor"/>
                                <rect x="2" y="2" width="1" height="1" fill="currentColor"/>
                                <rect x="3" y="2" width="1" height="1" fill="currentColor"/>
                                <rect x="4" y="2" width="1" height="1" fill="currentColor"/>
                                <rect x="5" y="2" width="1" height="1" fill="currentColor"/>
                                <rect x="0" y="3" width="1" height="1" fill="currentColor"/>
                                <rect x="1" y="3" width="1" height="1" fill="currentColor"/>
                                <rect x="2" y="3" width="1" height="1" fill="currentColor"/>
                                <rect x="3" y="3" width="1" height="1" fill="currentColor"/>
                                <rect x="4" y="3" width="1" height="1" fill="currentColor"/>
                                <rect x="5" y="3" width="1" height="1" fill="currentColor"/>
                            </svg>
                        </button>
                        <button class="titlebar-button maximize" onclick={maximize_onclick}>
                            <svg viewBox="0 0 6 6">
                                <rect x="0" y="0" width="1" height="1" fill="currentColor"/>
                                <rect x="1" y="0" width="1" height="1" fill="currentColor"/>
                                <rect x="2" y="0" width="1" height="1" fill="currentColor"/>
                                <rect x="3" y="0" width="1" height="1" fill="currentColor"/>
                                <rect x="4" y="0" width="1" height="1" fill="currentColor"/>
                                <rect x="5" y="0" width="1" height="1" fill="currentColor"/>
                                <rect x="0" y="1" width="1" height="1" fill="currentColor"/>
                                <rect x="5" y="1" width="1" height="1" fill="currentColor"/>
                                <rect x="0" y="2" width="1" height="1" fill="currentColor"/>
                                <rect x="5" y="2" width="1" height="1" fill="currentColor"/>
                                <rect x="0" y="3" width="1" height="1" fill="currentColor"/>
                                <rect x="5" y="3" width="1" height="1" fill="currentColor"/>
                                <rect x="0" y="4" width="1" height="1" fill="currentColor"/>
                                <rect x="5" y="4" width="1" height="1" fill="currentColor"/>
                                <rect x="0" y="5" width="1" height="1" fill="currentColor"/>
                                <rect x="1" y="5" width="1" height="1" fill="currentColor"/>
                                <rect x="2" y="5" width="1" height="1" fill="currentColor"/>
                                <rect x="3" y="5" width="1" height="1" fill="currentColor"/>
                                <rect x="4" y="5" width="1" height="1" fill="currentColor"/>
                                <rect x="5" y="5" width="1" height="1" fill="currentColor"/>
                            </svg>
                        </button>
                        <button class="titlebar-button close" onclick={close_onclick}>
                            <svg viewBox="0 0 6 6">
                                <rect x="0" y="0" width="1" height="1" fill="currentColor"/>
                                <rect x="5" y="0" width="1" height="1" fill="currentColor"/>
                                <rect x="1" y="1" width="1" height="1" fill="currentColor"/>
                                <rect x="4" y="1" width="1" height="1" fill="currentColor"/>
                                <rect x="2" y="2" width="1" height="1" fill="currentColor"/>
                                <rect x="3" y="3" width="1" height="1" fill="currentColor"/>
                                <rect x="1" y="4" width="1" height="1" fill="currentColor"/>
                                <rect x="4" y="4" width="1" height="1" fill="currentColor"/>
                                <rect x="0" y="5" width="1" height="1" fill="currentColor"/>
                                <rect x="5" y="5" width="1" height="1" fill="currentColor"/>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>

            {match &props.app_state {
                AppState::Dashboard(user_data) => html! {
                    <Preferences
                        user_data={user_data.clone()}
                        show={*show_preferences}
                        current_theme={props.current_theme.clone()}
                        on_close={close_preferences}
                        on_theme_change={props.on_theme_change.clone()}
                    />
                },
                AppState::Login => html! {},
            }}
        </>
    }
}
