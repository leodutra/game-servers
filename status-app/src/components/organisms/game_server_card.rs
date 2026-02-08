use leptos::prelude::*;
use crate::components::molecules::server_address::ServerAddress;
use crate::config::GameCardConfig;
use crate::components::atoms::bananil_spinner::BananilSpinner;
use crate::components::atoms::status_indicator::{StatusIndicator, Status};
use crate::components::atoms::stat_display::StatDisplay;

#[allow(non_snake_case)]
#[component]
pub fn GameServerCard(
    #[prop(into)] config: GameCardConfig,
    #[prop(into)] online: Signal<Option<bool>>,
    #[prop(into)] ping: Signal<String>,
    #[prop(into)] uptime: Signal<String>,
    #[prop(into)] uri: Signal<String>,
) -> impl IntoView {
    let inline_style = format!("background-image: url('{}')", config.background_image);

    view! {
        <div
            class=format!("p-6 bg-cover bg-center border-[5px] {} rounded-2xl shadow-card", config.border_class)
            style=inline_style
        >
            // Header
            <div class="flex justify-between items-center mb-3">
                <h2 class="text-4xl font-semibold text-white text-stroke-server [paint-order:stroke_fill] drop-shadow-brown-md">
                    {config.name}
                </h2>
                {move || match online.get() {
                    Some(true) => view! {
                        <StatusIndicator status=Status::Online />
                    }.into_any(),
                    Some(false) => view! {
                        <StatusIndicator status=Status::Offline />
                    }.into_any(),
                    None => view! {
                         <BananilSpinner />
                    }.into_any(),
                }}
            </div>

            // Content
            <div class="flex flex-wrap items-center gap-x-4 gap-y-2 mb-3">
                <div class="min-w-32 h-28 rounded-md overflow-hidden">
                    <img src=config.logo alt=format!("{} logo", config.name) class="w-full h-full object-contain" />
                </div>

                <div class="flex flex-1 justify-between items-center mr-1 min-w-[200px] flex-wrap gap-x-4">
                     <StatDisplay title="Ping:" value=ping />
                     <StatDisplay title="Uptime:" value=uptime />
                </div>
            </div>

            // URI
            <ServerAddress uri=uri />
        </div>
    }
}
