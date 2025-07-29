use crate::components::UserInfo;
use crate::states::{AppState, DashboardView};
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
    pub on_state_change: Callback<AppState>,
    pub on_view_change: Option<Callback<DashboardView>>,
}

#[function_component(Titlebar)]
pub fn titlebar(props: &TitlebarProps) -> Html {
    let show_user_info = use_state(|| false);

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
        let show_user_info = show_user_info.clone();
        Callback::from(move |_: MouseEvent| {
            show_user_info.set(false);
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.remove_item("user_email");
                    let _ = storage.remove_item("user_api_key");
                }
            }
            on_state_change.emit(AppState::Login);
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
        Callback::from(move |_| {
            show_user_info.set(false);
        })
    };

    html! {
        <>
            <div class="titlebar">
                <div class="titlebar-left">
                    if let AppState::Dashboard(_user_data, current_view) = &props.app_state {
                        <div class="titlebar-tabs">
                            <button
                                class={if matches!(current_view, DashboardView::Data) { "tab-button active" } else { "tab-button" }}
                                onclick={
                                    let on_view_change = props.on_view_change.clone();
                                    Callback::from(move |_: MouseEvent| {
                                        if let Some(callback) = &on_view_change {
                                            callback.emit(DashboardView::Data);
                                        }
                                    })
                                }
                            >
                                {"Data"}
                            </button>
                            <button
                                class={if matches!(current_view, DashboardView::Map) { "tab-button active" } else { "tab-button" }}
                                onclick={
                                    let on_view_change = props.on_view_change.clone();
                                    Callback::from(move |_: MouseEvent| {
                                        if let Some(callback) = &on_view_change {
                                            callback.emit(DashboardView::Map);
                                        }
                                    })
                                }
                            >
                                {"Map"}
                            </button>
                        </div>
                    }
                </div>

                <div class="titlebar-center">
                    if let AppState::Dashboard(_, _) = &props.app_state {
                        <span class="page-title">{"Dashboard"}</span>
                    }
                </div>

                <div class="titlebar-right">
                    if let AppState::Dashboard(_, _) = &props.app_state {
                        <button class="titlebar-nav-button user-info-button" onclick={toggle_user_info}>
                            {"User Info"}
                        </button>
                        <button class="titlebar-nav-button logout-button" onclick={on_logout}>
                            {"Logout"}
                        </button>
                    }

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

            {
                if let AppState::Dashboard(user_data, _) = &props.app_state {
                    html! {
                        <UserInfo
                            user_data={user_data.clone()}
                            show={*show_user_info}
                            on_close={close_user_info}
                        />
                    }
                } else {
                    html! {}
                }
            }
        </>
    }
}
