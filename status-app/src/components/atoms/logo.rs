use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <img
            src="images/logo.svg"
            alt="Cute smiling banana character mascot"
            class="flex-shrink-0 w-20 h-20 sm:w-24 sm:h-24 md:w-32 md:h-32 lg:w-40 lg:h-40 xl:w-44 xl:h-44 object-contain drop-shadow-md -mx-3 sm:-mx-4 md:-mx-6 lg:-mx-8"
        />
    }
}
