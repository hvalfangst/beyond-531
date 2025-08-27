use leptos::*;
use wasm_bindgen::prelude::*;

mod components;
mod beyond_531;

use components::Beyond531Calculator;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Beyond531Calculator />
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}