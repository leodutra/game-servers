use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Yellow,
    Green,
    Red,
    Hytale,
}

#[allow(non_snake_case)]
#[component]
pub fn Button(
    #[prop(default = ButtonVariant::Yellow)] variant: ButtonVariant,
    #[prop(into)] on_click: Callback<web_sys::MouseEvent>,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        ButtonVariant::Yellow => "bg-gradient-to-b from-yellow-btn-from to-yellow-btn-to hover:from-yellow-btn-hover-from hover:to-yellow-btn-hover-to",
        ButtonVariant::Green => "bg-gradient-to-b from-green to-green-to hover:brightness-110", // Approximation
        ButtonVariant::Red => "bg-gradient-to-b from-red to-red-to hover:brightness-110", // Approximation
        ButtonVariant::Hytale => "bg-hytale hover:brightness-110", // Approximation
    };

    view! {
        <button class=format!("px-4 py-2 flex items-center justify-center gap-2 transition flex-auto border-[3px] border-b-[2px] border-brown rounded-2xl shadow-btn font-bold text-xl text-white text-stroke-md [paint-order:stroke_fill] drop-shadow-brown-lg active:scale-95 {}", variant_class) on:click=move |e| on_click.run(e)>
            {children()}
        </button>
    }
}
