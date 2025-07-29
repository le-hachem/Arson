#[macro_use]
mod logging;

mod app;
mod components;
mod config;
mod errors;
mod services;
mod states;
mod types;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
