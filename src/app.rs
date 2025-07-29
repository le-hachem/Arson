use crate::components::Titlebar;
use crate::services::StorageService;
use crate::states::{dashboard::Dashboard, login::Login};
use crate::types::{AppState, DashboardView, Theme};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let app_state = use_state(|| AppState::Login);
    let dashboard_view = use_state(DashboardView::default);
    let current_theme = use_state(Theme::default);

    // Try to load saved credentials on app startup
    {
        let app_state = app_state.clone();
        use_effect_with((), move |_| {
            if let Ok(Some(user_data)) = StorageService::load_user_data() {
                app_state.set(AppState::Dashboard(user_data));
            }
            || {}
        });
    }

    // Apply theme to body class
    {
        let current_theme = current_theme.clone();
        use_effect_with((*current_theme).clone(), move |theme| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let body = document.body().unwrap();

            // Set theme class
            match theme {
                Theme::Light => {
                    let _ = body.set_class_name("theme-light");
                }
                Theme::Blue => {
                    let _ = body.set_class_name("theme-blue");
                }
                Theme::Dark => {
                    let _ = body.set_class_name("theme-dark");
                }
                Theme::Terminal => {
                    let _ = body.set_class_name("theme-terminal");
                }
            }

            || {}
        });
    }

    let on_state_change = {
        let app_state = app_state.clone();
        Callback::from(move |new_state: AppState| {
            app_state.set(new_state);
        })
    };

    let on_view_change = {
        let dashboard_view = dashboard_view.clone();
        Callback::from(move |new_view: DashboardView| {
            dashboard_view.set(new_view);
        })
    };

    let on_theme_change = {
        let current_theme = current_theme.clone();
        Callback::from(move |new_theme: Theme| {
            current_theme.set(new_theme);
        })
    };

    html! {
        <div class="app">
            <Titlebar
                app_state={(*app_state).clone()}
                dashboard_view={(*dashboard_view).clone()}
                current_theme={(*current_theme).clone()}
                on_state_change={on_state_change.clone()}
                on_view_change={on_view_change.clone()}
                on_theme_change={on_theme_change.clone()}
            />
            <main class="container">
                {match (*app_state).clone() {
                    AppState::Login => html! {
                        <Login on_state_change={on_state_change} />
                    },
                    AppState::Dashboard(user_data) => html! {
                        <Dashboard
                            user_data={user_data}
                            current_view={(*dashboard_view).clone()}
                            on_state_change={on_state_change}
                        />
                    }
                }}
            </main>
        </div>
    }
}
