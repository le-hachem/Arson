use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ResponseInfoProps {
    pub total_records: u32,
    pub returned_records: u32,
    #[prop_or(true)]
    pub success: bool,
}

#[function_component(ResponseInfo)]
pub fn response_info(props: &ResponseInfoProps) -> Html {
    html! {
        <div class="response-info">
            <p><strong>{"Success: "}</strong>{if props.success { "Yes" } else { "No" }}</p>
            <p><strong>{"Total Records: "}</strong>{props.total_records}</p>
            <p><strong>{"Returned Records: "}</strong>{props.returned_records}</p>
        </div>
    }
} 