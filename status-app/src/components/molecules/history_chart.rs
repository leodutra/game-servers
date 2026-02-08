use leptos::prelude::*;
use crate::components::atoms::history_bar::HistoryBar;
use crate::components::atoms::chart_tooltip::ChartTooltip;
use send_wrapper::SendWrapper;

#[allow(non_snake_case)]
#[component]
pub fn HistoryChart(
    #[prop(into)] history: Signal<Vec<(i64, bool)>>,
) -> impl IntoView {

    // Manage Tooltip State here (at molecule level)
    let tooltip_el = RwSignal::new(Option::<SendWrapper<web_sys::HtmlElement>>::None);

    let bars = move || {
        let history_data = history.get();
        // If history is empty, show empty state or some default
        let display_data = if history_data.is_empty() {
             // Show 48 gray bars? Or just 1?
             (0..48).map(|_| (0i64, true)).collect::<Vec<_>>()
        } else {
            // Take recent 48 or fill
             history_data.iter().rev().take(48).copied().collect::<Vec<_>>()
        };

        display_data.into_iter().map(|(ts, is_online)| {
            view! {
                <HistoryBar
                    timestamp=ts
                    is_online=is_online
                    tooltip_el=tooltip_el
                />
            }
        }).collect::<Vec<_>>()
    };

    view! {
        <div class="relative group/graph mt-16">
            <ChartTooltip tooltip_el_signal=tooltip_el />

             // Monkey Emoji
            <div class="absolute -top-20 right-6 z-20 transition-all group-hover/graph:-translate-y-2 drop-shadow-xl hover:scale-110 duration-300">
                <span class="text-7xl select-none" role="img" aria-label="monkey">"🐒"</span>
                <div class="absolute -top-2 -right-4 bg-green border-4 border-brown rounded-full px-2 py-0.5 shadow-sm transform rotate-12">
                    <span class="text-xl font-black text-white block leading-none uppercase">"WOW!"</span>
                </div>
            </div>

            // The Bevelled Graph Area
            <div id="uptime-bars-container" class="flex flex-row-reverse items-end justify-start gap-1.5 h-32 bg-banana-graph rounded-2xl p-6 relative overflow-hidden">
                <div class="absolute left-0 top-0 bottom-0 w-20 bg-[linear-gradient(to_right,theme('colors.banana.graph')_25%,transparent_100%)] z-10 pointer-events-none" style="pointer-events: none"></div>
                <div class="absolute inset-0 z-20 pointer-events-none rounded-2xl shadow-bevel" style="pointer-events: none"></div>
                {bars}
            </div>
        </div>
    }
}
