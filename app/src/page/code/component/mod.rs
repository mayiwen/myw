use crate::{
    myw::{
        self,
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::prelude::*;
pub mod button;
pub mod icon;
pub mod table;
/// Renders the home page of your application.
#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(1);
    view! {
        <myw::Gap h=8/>
        <h1>网站自用组件</h1>
        <myw::Gap h=8/>
        <Tabset id=id>
            <Tab slot id=1 title="表格".to_string()><table::I/></Tab>
            <Tab slot id=0 title="按钮".to_string()><button::I/></Tab>
            <Tab slot id=2 title="图标".to_string()><icon::I/></Tab>
        </Tabset>
    }
}
