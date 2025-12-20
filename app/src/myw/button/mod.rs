use leptos::prelude::*;

#[component]
pub fn I(
    children: Children,
    #[prop(optional, into)] border: MaybeSignal<String>,
    #[prop(optional, into)] active: MaybeSignal<bool>,
) -> impl IntoView {
    let border_style = move || {
        let border_value = border.get();
        if border_value == "both" {
            "1px solid var(--myw-border)".to_string()
        } else if border_value == "none" {
            "1px solid transparent".to_string()
        } else {
            "1px solid var(--myw-border)".to_string()
        }
    };
    view! {
        <button
            class="myw-button"
            style:border=border_style
            class:active=move ||active.get()
            >{children()}
        </button>
    }
}
