use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn StatDisplay(
    #[prop(into)] title: String,
    #[prop(into)] value: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="text-left flex-auto min-w-[140px]">
            <div class="text-2xl font-semibold text-white text-stroke-sm [paint-order:stroke_fill] drop-shadow-brown-sm">{title}</div>
            <div class="text-5xl font-semibold text-yellow text-stroke-stat [paint-order:stroke_fill] drop-shadow-brown-md leading-[1.2] min-h-[58px]">
                {value}
            </div>
        </div>
    }
}
