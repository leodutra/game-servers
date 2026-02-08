use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn UptimeTotal(
    #[prop(into)] uptime: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="text-center mb-10">
            <h2 class="flex flex-col sm:flex-row items-center justify-center gap-x-6 gap-y-2 mb-2 tracking-tight font-bold">
                <span class="text-4xl sm:text-5xl uppercase font-semibold text-white text-stroke-main [paint-order:stroke_fill] drop-shadow-brown-xl">
                    "Total Uptime:"
                </span>
                <span class="text-5xl sm:text-6xl text-green text-stroke-uptime [paint-order:stroke_fill] drop-shadow-green-xl">
                    {uptime}
                </span>
            </h2>

            <p class="text-2xl mt-6 font-bold font-medium text-yellow-light text-stroke-md [paint-order:stroke_fill] drop-shadow-brown-lg relative z-30 text-left sm:text-center">
                "Our servers are ape-solutely reliable!"
            </p>
        </div>
    }
}
