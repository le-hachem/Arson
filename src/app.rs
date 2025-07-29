use crate::components::Titlebar;
use crate::services::StorageService;
use crate::states::{dashboard::Dashboard, login::Login};
use crate::types::{AppState, DashboardView};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let app_state = use_state(|| AppState::Login);
    let dashboard_view = use_state(DashboardView::default);

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

    html! {
        <div class="app">
            <Titlebar
                app_state={(*app_state).clone()}
                dashboard_view={(*dashboard_view).clone()}
                on_state_change={on_state_change.clone()}
                on_view_change={on_view_change.clone()}
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
