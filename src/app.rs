use crate::titlebar::Titlebar;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="app">
            <Titlebar />
            <main class="container">
                <h1 id="title">{"Arson"}</h1>
                <p class="description">
                    {"Conflict and military event visualizer."}
                </p>

                <div class="form-container">
                    <div class="input-group">
                        <label for="email">{"Email Address:"}</label>
                        <input type="email" id="email" class="retro-input" placeholder="Enter your email" />
                    </div>

                    <div class="input-group">
                        <label for="api-key">{"ACLED API Key:"}</label>
                        <input type="password" id="api-key" class="retro-input" placeholder="Enter your ACLED API key" />
                    </div>

                    <button class="retro-button">{"Initialize"}</button>
                </div>
            </main>
        </div>
    }
}
