use crate::states::UserData;
use web_sys::MouseEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UserInfoProps {
    pub user_data: UserData,
    pub show: bool,
    pub on_close: Callback<()>,
}

#[function_component(UserInfo)]
pub fn user_info(props: &UserInfoProps) -> Html {
    let close_user_info = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    if props.show {
        html! {
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
    } else {
        html! {}
    }
}
