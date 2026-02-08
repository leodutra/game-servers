use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class="absolute top-0 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full text-center flex justify-center pointer-events-none">
             <div class="flex items-center justify-center gap-0 relative pointer-events-auto flex-nowrap">
                <img
                    src="images/logo.svg"
                    alt="Cute smiling banana character mascot with a happy face"
                    class="flex-shrink-0 w-20 h-20 sm:w-24 sm:h-24 md:w-32 md:h-32 lg:w-40 lg:h-40 xl:w-44 xl:h-44 object-contain drop-shadow-md -mx-3 sm:-mx-4 md:-mx-6 lg:-mx-8"
                />
                <h1
                    class="text-3xl sm:text-4xl md:text-5xl lg:text-6xl xl:text-7xl whitespace-nowrap font-semibold text-yellow text-stroke-main-osm md:text-stroke-main paint-stroke drop-shadow-brown-lg relative z-10 before:content-[attr(data-text)] before:absolute before:left-0 before:top-0 before:w-full before:h-full before:-z-10 before:text-stroke-main-outer-osm md:before:text-stroke-main-outer before:text-white before:[paint-order:stroke_fill]"
                    data-text="Bananil Server Status"
                > "Bananil Server Status"
                </h1>
            </div>
        </div>
    }
}
