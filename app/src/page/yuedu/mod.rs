use crate::{
    myw::{
        self,
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::prelude::*;

mod android;
mod ctrl;
mod ios;
mod linux;
mod macos;
mod windows;
/// Renders the home page of your application.
#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);
    view! {
        <div style="text-align: center">
            <p style="text-align: right; margin-right: 8px;">
                <a href="https://mayiwen.com/yueduqi/yinsishengming"
                    title="https://mayiwen.com/yueduqi/yinsishengming">隐私声明1</a>
            </p>
            <div style="text-align: center; max-width: 400px; margin: auto;">
                <myw::Gap h=12/>
                <h1 style="font-weight: bold;">一文小说阅读器</h1>
                <myw::Gap h=12/>
                <myw::button::I style="transform: scale(1.2); transform-origin: center"  on:click=move |_| {
                    open_url("https://gitlink.org.cn/mayiwen/yueduqi/releases");
                }>软件与说明书下载</myw::button::I>
                <myw::Gap  h=12/>
                <a href="https://github.com/mayiwen/yueduqi/releases" target="_blank">github分流下载</a>
                <myw::Gap h=12/>
                <div style="text-align: left;">
                    <Tabset id=id>
                        <Tab slot id=0 title="ios".to_string()><ios::I/></Tab>
                        <Tab slot id=1 title="android".to_string()><android::I/></Tab>
                        <Tab slot id=2 title="windows".to_string()><windows::I/></Tab>
                        <Tab slot id=3 title="macos".to_string()><macos::I/></Tab>
                        <Tab slot id=4 title="linux".to_string()><linux::I/></Tab>
                        <Tab slot id=5 title="功能".to_string() ><ctrl::I/></Tab>
                    </Tabset>
                </div>
            </div>

        </div>
    }
}
