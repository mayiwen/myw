use crate::myw;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
/// Renders the home page of your application.
#[component]
pub fn I() -> impl IntoView {
    // let (border, set_border) = signal::<String>("none".to_string());
    // let on_click = move |_| {
    //     if border.get() == "none".to_string() {
    //         set_border.set("both".to_string())
    //     } else {
    //         set_border.set("none".to_string())
    //     };
    // };
    // <Button on:click=on_click >点击</Button>
    // <Button border=border>你好</Button>
    // <Button border="none">你好</Button>
    // <Button active=true>你好</Button>
    view! {
        "这是阅读的页面"
    }
}
