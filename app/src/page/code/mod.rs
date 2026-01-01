use crate::{
    myw::{
        self,
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::prelude::*;
pub mod client;
pub mod component;
pub mod firefox;
pub mod input;
/// Renders the home page of your application.
#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);
    view! {
        <myw::Gap h=8/>
        <div>
            <h1 style="display: inline-block;">代码</h1>
            <h3 style="display: inline-block;">自己用的软件</h3>
        </div>
        <myw::Gap h=8/>
        <Tabset id=id>
            <Tab slot id=0 title="自制组件".to_string()><component::I/></Tab>
            <Tab slot id=1 title="客户端".to_string()><client::I/></Tab>
            <Tab slot id=2 title="输入法".to_string()><input::I/></Tab>
            <Tab slot id=3 title="火狐插件".to_string()><firefox::I/></Tab>
        </Tabset>


    }
}
