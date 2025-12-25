use crate::{
    myw::{
        self,
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::prelude::*;
use myw::button;
/// Renders the home page of your application.
#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);
    view! {
        <myw::Gap h=8/>
        <button::I>基础按钮</button::I> <myw::Gap w=8/>
        <button::I active=true>选中按钮</button::I> <myw::Gap w=8/>
        <button::I border="none">无边框按钮</button::I>
    }
}
