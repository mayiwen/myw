use crate::{
    myw::{
        self,
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::prelude::*;
pub mod button;
pub mod table;
/// Renders the home page of your application.
#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);
    view! {
        <myw::Gap h=8/>
        <h1>网站自用组件</h1>
        <myw::Gap h=8/>
        <Tabset id=id>
            <Tab slot id=0 title="按钮".to_string()><button::I/></Tab>
            <Tab slot id=1 title="表格".to_string()><table::I/></Tab>
            // <Tab slot id=2 title="输入法".to_string()><input::I/></Tab>
            // <Tab slot id=3 title="火狐插件".to_string()><firefox::I/></Tab>
            // <Tab slot id=1 title="android".to_string()><android::I/></Tab>
            // <Tab slot id=2 title="windows".to_string()><windows::I/></Tab>
            // <Tab slot id=3 title="macos".to_string()><macos::I/></Tab>
            // <Tab slot id=4 title="linux".to_string()><linux::I/></Tab>
            // <Tab slot id=5 title="功能".to_string() ><ctrl::I/></Tab>
        </Tabset>
    }
}
