use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(Titlebar)]
pub fn titlebar() -> Html {
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

    html! {
        <div class="titlebar">
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
    }
}
