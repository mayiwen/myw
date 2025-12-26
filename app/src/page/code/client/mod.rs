use crate::{
    myw::{
        self,
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::prelude::*;
/// Renders the home page of your application.
#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);
    view! {
        <div style="text-align: center">
             <myw::Gap/>
            <h1>
                马一文桌面端
            </h1>
            <myw::Gap h=30/>
            <myw::button::I style="transform: scale(1.4); transform-origin: center"  on:click=move |_| {
                open_url("https://gitlink.org.cn/mayiwen/nuoruo/releases");
            }>获取开源软件</myw::button::I>
            <myw::Gap h=30/>
            <div>
                <img style="width: 100%; max-width: 1000px;" src="img/pc/mayiwen.png" alt="主界面" />
            </div>
            <div>
                <img style="width: 100%; max-width: 1000px;" src="img/pc/browser.png" alt="浏览器界面" />
            </div>
            <p>"0.4 版本在使用rust tauri 重置，现此版本已废弃"</p>
            <p>做一个类似浏览器的软件</p>
        </div>
    }
}
