use crate::{
    myw::{
        self,
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::prelude::*;
#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);
    view! {
        <myw::Gap/>
        <h1>设置与关于</h1>
        <myw::Gap/>
        <Tabset id=id>
            <Tab slot id=0 title="首页标题".to_string()>""</Tab>
            <Tab slot id=1 title="链接".to_string()>""</Tab>
            <Tab slot id=2 title="关于".to_string()>""</Tab>
            // <Tab slot id=1 title="android".to_string()><android::I/></Tab>
            // <Tab slot id=2 title="windows".to_string()><windows::I/></Tab>
            // <Tab slot id=3 title="macos".to_string()><macos::I/></Tab>
            // <Tab slot id=4 title="linux".to_string()><linux::I/></Tab>
            // <Tab slot id=5 title="功能".to_string() ><ctrl::I/></Tab>
        </Tabset>
    }
}
