use crate::{
    myw::{
        self,
        button::Button,
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
                火狐插件 mayiwen
            </h1>

            <myw::Gap h=30/>
            <Button style="transform: scale(1.4); transform-origin: center"  on_click=move |_| {
                open_url("https://addons.mozilla.org/zh-CN/firefox/addon/mayiwen/");
            }>获取开源软件</Button>

            <myw::Gap h=30/>
              <p>"可以火狐插件中搜索“mayiwen”找到此插件"</p>
            <p>"实现的功能："</p>
            <p>"1. 点击插件的图标，可以关闭其他标签页、关闭左侧标签页、关闭右侧标签页。"</p>
            <p>"2. 在标签处右键，出现关闭其他标签选项。"</p>
            <p>"3. 在网页右键，出现关闭其他标签选项。"</p>
            <p>"4. 使用快捷键 CTRL + Q，关闭其他标签。"</p>
             <myw::Gap />
            <div>
                <img style="width: 100%; max-width: 1000px;" src="img/firefox/firefox.png" alt="主界面" />
            </div>
        </div>
    }
}
