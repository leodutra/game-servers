use leptos::prelude::*;
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsValue;

#[allow(non_snake_case)]
#[component]
pub fn HistoryBar(
    #[prop(into)] timestamp: i64,
    #[prop(into)] is_online: bool,
    #[prop(into)] tooltip_el: Signal<Option<SendWrapper<web_sys::HtmlElement>>>,
) -> impl IntoView {
    let bar_color = if is_online { "#FFD633" } else { "#8D6E63" };
    let stroke_color = if is_online { "#583017" } else { "#3E2723" };
    let height = if is_online { "60px" } else { "40px" };
    let opacity = if is_online { "1" } else { "0.7" };
    let status_text = if is_online { "OPERATIONAL" } else { "OUTAGE" };

    let inline_style = format!(
        "height: {}; background-color: {}; border-color: {}; opacity: {}",
        height, bar_color, stroke_color, opacity
    );

    // Date Calculation
    let date = if timestamp > 0 {
        js_sys::Date::new(&JsValue::from_f64(timestamp as f64))
    } else {
        js_sys::Date::new_0()
    };

    let pad = |n: f64| format!("{:02}", n as u32);
    let timestamp_str = format!(
        "{}-{}-{} {}:{}",
        date.get_full_year(),
        pad(date.get_month() as f64 + 1.0),
        pad(date.get_date() as f64),
        pad(date.get_hours() as f64),
        pad(date.get_minutes() as f64)
    );

    let tooltip_str = format!("{} • {}", timestamp_str, status_text);

    // Event Handlers
    let on_enter = move |ev: MouseEvent| {
         if let Some(target) = ev.current_target() {
            let el = target.unchecked_into::<web_sys::Element>();
            let rect = el.get_bounding_client_rect();

            // Use VIEWPORT coordinates directly
            let left = rect.left() + rect.width() / 2.0;
            let top = rect.top();

             if let Some(tooltip) = tooltip_el.get() {
                 let _ = tooltip.set_attribute("style", &format!(
                     "top: {}px; left: {}px; transform: translate(-50%, -100%); margin-top: -12px; opacity: 1; pointer-events: none;",
                     top, left
                 ));
                 tooltip.set_text_content(Some(&tooltip_str));
             }
        }
    };

    let on_leave = move |_| {
         if let Some(tooltip) = tooltip_el.get() {
               let _ = tooltip.set_attribute("style", "opacity: 0; pointer-events: none; transform: translate(-50%, -100%); margin-top: -12px;");
         }
    };

    view! {
        <div
            class="flex-none w-3 group/bar relative flex flex-col items-center cursor-help z-0"
            on:mouseenter=on_enter
            on:mouseleave=on_leave
        >
            <div
                class="w-full transition-all duration-300 transform group-hover/bar:-translate-y-1 group-hover/bar:drop-shadow-md rounded-full border-2 border-b-4 shadow-sm"
                style=inline_style
            ></div>
        </div>
    }
}
