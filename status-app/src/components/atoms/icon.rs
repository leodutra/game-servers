use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Icon(
    #[prop(into)] name: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <i class=format!("fas fa-{} {}", name, class)></i>
    }
}
