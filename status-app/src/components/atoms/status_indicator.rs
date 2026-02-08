use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Status {
    Online,
    Offline,
    Starting,
    Maintenance,
}

#[allow(non_snake_case)]
#[component]
pub fn StatusIndicator(status: Status) -> impl IntoView {
    let (status_text, status_class, icon_class, icon_bg, icon_text_color) = match status {
        Status::Online => ("Online", "bg-gradient-to-b from-green to-green-to border-green-border text-stroke-green-sm", "fa-check", "bg-green-light", "text-green"),
        Status::Offline => ("Offline", "bg-gradient-to-b from-red to-red-to border-red-border text-stroke-red-sm", "fa-times", "bg-red-light", "text-red"),
        Status::Starting => ("Starting", "bg-yellow border-brown text-stroke-sm", "fa-hourglass-half", "bg-yellow-light", "text-brown"), // Fallback style
        Status::Maintenance => ("Maintenance", "bg-hytale border-brown text-stroke-sm", "fa-tools", "bg-banana-card", "text-hytale"), // Fallback style
    };

    view! {
        <div class=format!("text-white px-2 py-1.5 rounded-full flex items-center gap-2 border-[3px] font-semibold [paint-order:stroke_fill] pr-4 {}", status_class)>
            <div class=format!("w-7 h-7 rounded-full flex items-center justify-center shadow-inner {}", icon_bg)>
                <i class=format!("fas {} text-sm {} drop-shadow-sm", icon_class, icon_text_color)></i>
            </div>
            <span class="text-xl leading-none pt-0.5">{status_text}</span>
        </div>
    }
}
