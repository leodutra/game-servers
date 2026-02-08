use leptos::prelude::*;
use leptos::prelude::set_timeout;
use crate::components::atoms::button::{Button, ButtonVariant};
use wasm_bindgen::JsCast;

#[component]
pub fn ServerAddress(#[prop(into)] uri: Signal<String>) -> impl IntoView {
    let copy_uri = uri.clone();
    let (copied, set_copied) = signal(false);

    // Initial check for clipboard support (secure context)
    let is_clipboard_available = if let Some(window) = web_sys::window() {
        if let Ok(val) = js_sys::Reflect::get(&window.navigator(), &"clipboard".into()) {
            !val.is_undefined() && !val.is_null()
        } else {
            false
        }
    } else {
        false
    };

    let on_copy = move |_| {
        // We only show the button if available, so unwrap/expects would be safer here,
        // but we'll stick to safe checks.
        if let Some(window) = web_sys::window() {
             let navigator = window.navigator();
             if let Ok(val) = js_sys::Reflect::get(&navigator, &"clipboard".into()) {
                 if let Ok(clipboard) = val.dyn_into::<web_sys::Clipboard>() {
                     let _ = clipboard.write_text(&copy_uri.get());
                     set_copied.set(true);
                     set_timeout(move || set_copied.set(false), std::time::Duration::from_millis(2000));
                 }
             }
        }
    };

    view! {
        <div>
            <div class="text-xl mb-2 font-semibold text-white text-stroke-sm [paint-order:stroke_fill] drop-shadow-brown-sm">"URI:"</div>
            <div class="flex flex-wrap items-stretch gap-2">
                <div class="px-2 py-2 text-xl flex items-center justify-center flex-[10_1_300px] bg-banana-input border-[3px] border-transparent rounded-2xl text-white font-medium text-stroke-sm [paint-order:stroke_fill] drop-shadow-brown-xs">
                    {uri}
                </div>
                {if is_clipboard_available {
                    Some(view! {
                        <Button
                            on_click=on_copy
                            variant=ButtonVariant::Yellow
                        >
                            {move || if copied.get() {
                                view! {
                                    <i class="fas fa-check"></i>
                                    "Copied!"
                                }.into_any()
                            } else {
                                view! {
                                    <i class="fas fa-clipboard"></i>
                                    "Copy"
                                }.into_any()
                            }}
                        </Button>
                    })
                } else {
                    None
                }}
            </div>
        </div>
    }
}
