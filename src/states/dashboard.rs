use crate::states::{dataview::DataView, AppState, UserData};
use web_sys::MouseEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DashboardProps {
    pub user_data: UserData,
    pub on_state_change: Callback<AppState>,
    pub show_user_info: bool,
    pub on_close_user_info: Callback<()>,
}

#[function_component(Dashboard)]
pub fn dashboard(props: &DashboardProps) -> Html {
    let close_user_info = {
        let on_close_user_info = props.on_close_user_info.clone();
        Callback::from(move |_: MouseEvent| {
            on_close_user_info.emit(());
        })
    };

    html! {
        <>
            <div class="dashboard">
                <DataView user_data={props.user_data.clone()} />
            </div>

            if props.show_user_info {
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
        </>
    }
}
