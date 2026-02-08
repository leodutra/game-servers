use leptos::prelude::*;
use status_app::components::pages::home::Home;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <Home/> })
}
