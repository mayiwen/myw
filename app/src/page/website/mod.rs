use crate::{
    myw::{
        self,
        button::Button,
        icon,
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::prelude::*;

mod android;
mod ios;
mod linux;
mod macos;
mod windows;

#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);
    view! {
      <myw::Gap h=8 />
      <div style=" max-width: 400px; margin: auto; text-align: center">
        <myw::Gap h=30 />
        <div>
          <img
          //  invert(35%) sepia(81%) saturate(2200%) hue-rotate(332deg) brightness(99%) contrast(101%)
            style="max-width: 100px;filter: "
            src="img/nuoruo/nuoruo_logo_square.png"
            alt="nuoruo logo"
          />
        </div>
        <h2>"诺若浏览器"</h2>
        <p>"https://mayiwen.com/browser"</p>
        // <p>"基于rust、wry、tauri、leptos构建"</p>
        // <a href="https://github.com/mayiwen/nuoruo_browser" target="_blank">
        //   "https://github.com/mayiwen/nuoruo_browser"
        // </a>
        <myw::Gap h=30 />
        <div style="text-align: left;">
          <Tabset id=id>
            <Tab slot id=0 title="ios".to_string()><ios::I/></Tab>
            <Tab slot id=1 title="android".to_string()><android::I/></Tab>
            <Tab slot id=2 title="windows".to_string()><windows::I/></Tab>
            <Tab slot id=3 title="macos".to_string()><macos::I/></Tab>
            <Tab slot id=4 title="linux".to_string()><linux::I/></Tab>
          </Tabset>
        </div>
        <myw::Gap h=30 />
        <p>"nuoruo网站暂未开放"</p>
        <p>"需访问mayiwen.com"</p>
        <myw::Gap h=30 />
      </div>
    }
}
