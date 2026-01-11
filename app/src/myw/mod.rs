use leptos::prelude::*;
pub mod button;
pub mod icon;
pub mod input;
pub mod message;
pub mod modal;
pub mod myw;
pub mod table;
pub mod tabset;

#[component]
pub fn Gap(
    #[prop(optional)] w: Option<u32>,         // 像素值
    #[prop(optional)] h: Option<u32>,         // 像素值
    #[prop(optional)] width: Option<String>,  // 带单位的字符串
    #[prop(optional)] height: Option<String>, // 带单位的字符串
) -> impl IntoView {
    let style = match (w, h, width, height) {
        (Some(w), _, _, _) => format!("display: inline-block; width: {}px", w),
        (_, Some(h), _, _) => format!("height: {}px", h),
        (_, _, Some(width), _) => format!("display: inline-block; width: {}", width),
        (_, _, _, Some(height)) => format!("height: {}", height),
        _ => "height: 8px".to_string(),
    };
    view! { <div style=style></div> }
}
#[component]
pub fn I() -> impl IntoView {
    view! {
        // <tabset::Test></tabset::Test>
    }
}
