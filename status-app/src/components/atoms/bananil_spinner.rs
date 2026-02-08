use leptos::prelude::*;

#[component]
pub fn BananilSpinner() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center p-2">
            <span class="text-4xl animate-bananil-spin inline-block">"🍌"</span>
        </div>
    }
}
