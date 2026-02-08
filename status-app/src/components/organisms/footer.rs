use leptos::prelude::*;
use crate::components::atoms::uptime_total::UptimeTotal;
use crate::components::molecules::history_chart::HistoryChart;

#[allow(non_snake_case)]
#[component]
pub fn Footer(
    #[prop(into)] uptime: Signal<String>,
    #[prop(into)] history: Signal<Vec<(i64, bool)>>,
) -> impl IntoView {

    view! {
        <div class="max-w-4xl w-full mx-auto mt-12 relative z-10 text-brown px-4">
             // Header section (Atom)
             <UptimeTotal uptime=uptime />

             // The Bar Graph Container (Molecule)
            <HistoryChart history=history />

            // Graph Legend / Footer
            <div class="flex flex-col sm:flex-row justify-between items-center mt-8 gap-4 px-2">
                <div class="flex items-center gap-2 text-sm uppercase font-semibold text-white text-stroke-sm [paint-order:stroke_fill] drop-shadow-brown-sm">
                     <i class="far fa-clock text-lg text-white drop-shadow-brown-sm"></i>
                     <span>"Recent History"</span>
                </div>

                <div class="flex gap-6">
                    <div class="flex items-center gap-2">
                        <div class="w-4 h-4 rounded-full bg-yellow border-2 border-brown"></div>
                        <span class="font-bold text-brown text-xs uppercase tracking-wider" style="text-shadow: 0 1px 0 rgba(0,0,0,0.1)">"Online"</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <div class="w-4 h-4 rounded-full bg-brown-light border-2 border-brown"></div>
                        <span class="font-bold text-brown text-xs uppercase tracking-wider" style="text-shadow: 0 1px 0 rgba(0,0,0,0.1)">"Incident"</span>
                    </div>
                </div>

                <div class="flex items-center gap-2 text-sm uppercase font-semibold text-white text-stroke-sm [paint-order:stroke_fill] drop-shadow-brown-sm">
                    <i class="fas fa-wifi text-lg animate-pulse drop-shadow-brown-sm text-white"></i>
                    <span>"Live System"</span>
                </div>
            </div>

             // Copyright Footer
            <div class="text-center text-gray-500 text-base mt-4 font-fredoka font-medium">
                "Bananil Servers © 2026 | Powered by potassium & pixels"
            </div>
        </div>
    }
}

