use leptos::*;
use leptos::logging;
use std::collections::HashMap;
use shared::ServerStatus;
use gloo_events::EventListener;
use wasm_bindgen::JsCast;

#[component]
fn ServerCard(status: ServerStatus) -> impl IntoView {
    let status_color = if status.online { "bg-emerald-600" } else { "bg-red-600" };

    view! {
        <div class={format!("p-6 rounded-xl shadow-lg text-white transition-all duration-300 hover:scale-105 {}", status_color)}>
            <div class="flex justify-between items-start mb-4">
                <div>
                    <h2 class="text-2xl font-bold">{status.name}</h2>
                    <span class="text-xs uppercase tracking-wider opacity-80">{status.server_type}</span>
                </div>
                <div class="text-right">
                    <div class="text-xs opacity-75">Last Updated</div>
                    <div class="font-mono text-sm">{status.last_updated}</div>
                </div>
            </div>

            <div class="space-y-2">
                <div class="flex items-center gap-2">
                    <span class="text-xl">{if status.online { "● Online" } else { "○ Offline" }}</span>
                </div>

                <div class="flex justify-between items-center bg-black/20 p-2 rounded">
                    <span>"Players"</span>
                    <span class="font-mono font-bold">{status.players}</span>
                </div>

                <div class="text-sm opacity-90 italic">
                    {status.details}
                </div>
            </div>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let (servers, set_servers) = create_signal(HashMap::<String, ServerStatus>::new());

    create_effect(move |_| {
        // In development, might need full URL if ports differ, but here we assume served by backend
        let event_source = web_sys::EventSource::new("/api/events").unwrap();

        let on_message = EventListener::new(&event_source, "message", move |event| {
            if let Ok(msg_event) = event.clone().dyn_into::<web_sys::MessageEvent>() {
                if let Some(text) = msg_event.data().as_string() {
                     match serde_json::from_str::<ServerStatus>(&text) {
                        Ok(new_status) => {
                            set_servers.update(|map| {
                                map.insert(new_status.name.clone(), new_status);
                            });
                        },
                        Err(e) => logging::log!("Failed to parse JSON: {:?}", e),
                    }
                }
            }
        });

        on_cleanup(move || drop(on_message));
    });

    view! {
        <div class="min-h-screen bg-slate-900 p-8 font-sans">
            <h1 class="text-4xl font-black text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-emerald-400 mb-8">
                "Sentinel Dashboard"
            </h1>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <For
                    each={move || servers.get().into_values().collect::<Vec<_>>()}
                    key={|s| s.name.clone()}
                    children={move |status| view! { <ServerCard status=status /> }}
                />
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
