use leptos::prelude::*;
pub mod button;
pub mod icon;
pub mod input;
pub mod myw;
pub mod table;
pub mod tabset;

#[component]
pub fn Gap(
    #[prop(optional)] w: &'static str,
    #[prop(optional)] h: &'static str,
    #[prop(optional)] width: &'static str,
    #[prop(optional)] height: &'static str,
) -> impl IntoView {
    let style = if !w.is_empty() {
        format!("display: inline-block; width: {}px", w)
    } else if !h.is_empty() {
        format!("height: {}px", h)
    } else if !width.is_empty() {
        format!("display: inline-block; width: {}", width)
    } else if !height.is_empty() {
        format!("height: {}", height)
    } else {
        format!("height: 30px{}", "")
    };
    view! { <div style=style></div> }
}

#[component]
pub fn I() -> impl IntoView {
    view! {
        // <tabset::Test></tabset::Test>
    }
}
