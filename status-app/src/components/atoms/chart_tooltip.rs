use leptos::prelude::*;
use wasm_bindgen::JsCast;
use send_wrapper::SendWrapper;

#[allow(non_snake_case)]
#[component]
pub fn ChartTooltip(
    #[prop(into)] tooltip_el_signal: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
) -> impl IntoView {
    // This component purely manages the existence of the tooltip DOM element
    // The actual "showing" is handled by the parent/bar on hover, using the signal.

    Effect::new(move |_| {
        if let Some(win) = web_sys::window()
            && let Some(doc) = win.document()
                && let Some(body) = doc.body() {
                    let div = doc.create_element("div").expect("div").unchecked_into::<web_sys::HtmlElement>();

                    // Exact class string from bananil.html
                    div.set_class_name("fixed opacity-0 pointer-events-none transition-opacity duration-200 bg-brown text-white text-[14px] py-1.5 px-4 rounded-full z-[9999] whitespace-nowrap shadow-xl font-bold border-2 border-white/20 font-mono ring-4 ring-brown/10");
                    // Initial styles
                    let _ = div.set_attribute("style", "transform: translate(-50%, -100%); margin-top: -12px; pointer-events: none;");

                    let _ = body.append_child(&div);
                    tooltip_el_signal.set(Some(SendWrapper::new(div.clone())));

                    // Cleanup
                    let cleanup_div = SendWrapper::new(div.clone());
                    on_cleanup(move || {
                        cleanup_div.remove();
                    });
                }
    });

    view! { } // Renders nothing in the flow
}
