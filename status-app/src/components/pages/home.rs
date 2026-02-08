use leptos::prelude::*;
use crate::components::organisms::header::Header;
use crate::components::organisms::game_server_card::GameServerCard;
use crate::components::organisms::footer::Footer;
use crate::models::{History, PublicConfig};
use crate::config::{TERRARIA_CONFIG, HYTALE_CONFIG};
use web_sys::{EventSource, MessageEvent};
use wasm_bindgen::prelude::*;
use gloo_net::http::Request;
use chrono::DateTime;

use crate::components::atoms::bananil_spinner::BananilSpinner;

fn parse_history_from_csv(csv_str: &str) -> History {
    let mut history = History::default();
    for (i, line) in csv_str.lines().enumerate() {
        if i == 0 { continue; }
        if line.trim().is_empty() { continue; }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 5 {
            if let Ok(ts) = DateTime::parse_from_rfc3339(parts[0]) {
                let timestamp = ts.with_timezone(&chrono::Utc);
                // Terraria
                let t_online = match parts[1].trim() {
                    "true" => Some(true),
                    "false" => Some(false),
                    _ => None,
                };
                let t_latency = parts[2].parse().unwrap_or(0);

                // Hytale
                let h_online = match parts[3].trim() {
                    "true" => Some(true),
                    "false" => Some(false),
                    _ => None,
                };
                let h_latency = parts[4].parse().unwrap_or(0);

                history.terraria.push(crate::models::ServiceStatus {
                    service_name: "Terraria".to_string(),
                    is_online: t_online,
                    latency_ms: t_latency,
                    timestamp,
                });
                history.hytale.push(crate::models::ServiceStatus {
                    service_name: "Hytale".to_string(),
                    is_online: h_online,
                    latency_ms: h_latency,
                    timestamp,
                });
            }
        }
    }
    history
}

fn calculate_service_stats(entries: &[crate::models::ServiceStatus]) -> (String, String, Option<bool>, usize, usize) {
    if entries.is_empty() {
        return ("---".to_string(), "---".to_string(), Some(false), 0, 0);
    }

    let valid_entries: Vec<_> = entries.iter().filter(|s| s.is_online.is_some()).collect();
    let total = valid_entries.len();
    let online = valid_entries.iter().filter(|s| s.is_online == Some(true)).count();

    let uptime = if total > 0 {
        format!("{:.1}%", (online as f64 / total as f64) * 100.0)
    } else {
        "---".to_string()
    };

    let (ping, status) = if let Some(last) = entries.last() {
        (format!("{}ms", last.latency_ms), last.is_online)
    } else {
        ("---".to_string(), Some(false))
    };

    (uptime, ping, status, total, online)
}

fn generate_history_bars(history: &History) -> Vec<(i64, bool)> {
    let len = std::cmp::min(history.terraria.len(), history.hytale.len());
    let mut bars = Vec::with_capacity(len);
    for i in 0..len {
         let t = &history.terraria[i];
         let h = &history.hytale[i];
         let t_status = t.is_online.unwrap_or(true);
         let h_status = h.is_online.unwrap_or(true);
         let ts = t.timestamp.timestamp_millis();
         bars.push((ts, t_status && h_status));
    }
    bars
}

#[allow(non_snake_case)]
#[component]
pub fn Home() -> impl IntoView {
    // Fetch system config (URIs)
    let config = LocalResource::new(|| async move {
        if let Ok(resp) = Request::get("/api/config").send().await {
            if let Ok(cfg) = resp.json::<PublicConfig>().await {
                 return Some(cfg);
            }
        }
        None
    });

    // Signals to store the latest status
    let (terraria_online, set_terraria_online) = signal(Option::<bool>::None);
    let (terraria_ping, set_terraria_ping) = signal("---".to_string());
    let (terraria_uptime, set_terraria_uptime) = signal("---".to_string());

    let (hytale_online, set_hytale_online) = signal(Option::<bool>::None);
    let (hytale_ping, set_hytale_ping) = signal("---".to_string());
    let (hytale_uptime, set_hytale_uptime) = signal("---".to_string());

    let (total_uptime, set_total_uptime) = signal("---".to_string());
    // Vec<(timestamp_ms, is_online)>
    let (history_bars, set_history_bars) = signal(Vec::<(i64, bool)>::new());
    let (has_loaded_history, set_has_loaded_history) = signal(false);

    // Connect to SSE
    Effect::new(move |_| {
        let event_source = EventSource::new("/api/sse").expect("Failed to connect to SSE");

        let onmessage = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                set_has_loaded_history.set(true);
                let csv_str: String = txt.into();

                // Use helper to parse
                let history = parse_history_from_csv(&csv_str);

                // Update Terraria
                let (t_uptime, t_ping, t_status, t_checks, t_online) = calculate_service_stats(&history.terraria);
                set_terraria_uptime.set(t_uptime);
                set_terraria_ping.set(t_ping);
                set_terraria_online.set(t_status);

                // Update Hytale
                let (h_uptime, h_ping, h_status, h_checks, h_online) = calculate_service_stats(&history.hytale);
                set_hytale_uptime.set(h_uptime);
                set_hytale_ping.set(h_ping);
                set_hytale_online.set(h_status);

                // Total Uptime
                let total_checks = t_checks + h_checks;
                let total_online = t_online + h_online;
                if total_checks > 0 {
                    let pct = (total_online as f64 / total_checks as f64) * 100.0;
                    set_total_uptime.set(format!("{:.2}%", pct));
                } else {
                    set_total_uptime.set("---".to_string());
                }

                // History Bars
                let bars = generate_history_bars(&history);
                set_history_bars.set(bars);
            }
        });

        event_source.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        onmessage.forget(); // Keep the closure alive
    });

    view! {
        // Outer wrapper simulating body
        <div class="min-h-screen flex flex-col items-center py-16 px-4 font-fredoka relative before:content-[''] before:fixed before:top-0 before:left-0 before:w-full before:h-full before:bg-banana-bg before:bg-banana-tile before:bg-repeat before:bg-[length:800px] before:opacity-80 before:-z-10 before:pointer-events-none">

            // Main Content Card
            <div class="w-full max-w-6xl rounded-3xl shadow-xl p-8 pb-6 relative z-10 mt-12 bg-banana-card">
                <Header />

                // Server Cards Container
                <div class="grid grid-cols-1 min-[900px]:grid-cols-2 gap-6 mb-8 mt-4 md:mt-14">
                    <GameServerCard
                        config=TERRARIA_CONFIG
                        online=terraria_online
                        ping=terraria_ping
                        uptime=terraria_uptime
                        uri=move || config.get().flatten().map(|c| c.terraria.clone()).unwrap_or("...".to_string())
                    />
                    <GameServerCard
                        config=HYTALE_CONFIG
                        online=hytale_online
                        ping=hytale_ping
                        uptime=hytale_uptime
                        uri=move || config.get().flatten().map(|c| c.hytale.clone()).unwrap_or("...".to_string())
                    />
                </div>

                {move || if has_loaded_history.get() {
                    view! { <Footer uptime=total_uptime history=history_bars /> }.into_any()
                } else {
                     view! {
                        <div class="w-full flex justify-center py-6">
                            <BananilSpinner />
                        </div>
                     }.into_any()
                }}
            </div>
        </div>
    }
}

