use leptos::*;

mod app;
use app::App;
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}
