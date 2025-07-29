use crate::states::UserData;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MapProps {
    pub user_data: UserData,
}

#[function_component(Map)]
pub fn map(_props: &MapProps) -> Html {
    html! {
        <div class="map-container">
            <div class="map-empty">
                <p>{"Coming soon"}</p>
            </div>
        </div>
    }
}
