use leptos::*;

mod app;
mod components;

use app::App;

pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}
