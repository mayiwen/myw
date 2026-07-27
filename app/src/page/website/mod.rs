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
pub mod yinsishengming;
use leptos_router::components::Outlet;

#[component]
pub fn I() -> impl IntoView {
    view! {
        <Outlet/>
    }
}

#[component]
pub fn Index() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);
    let global_nav =
        use_context::<RwSignal<Option<crate::NavFn>>>().expect("请确保 App 组件已提供全局导航信号");
    view! {
      <myw::Gap h=8 />
          <div style="text-align: right; margin-right: 8px;">
                <a
                    style="text-decoration: underline; cursor: pointer; line-height: 40px;"
                    on:click= move |_| {
                        global_nav.with(|nav_opt| {
                            if let Some(nav) = nav_opt {
                                nav("/browser/yinsishengming");
                            };
                        });
                    }
                    title="https://mayiwen.com/browser/yinsishengming">
                        隐私声明
                </a>
                <myw::Gap w=8/>
                <Button  on_click=move |_| {
                    open_url("https://github.com/mayiwen/nuoruo_browser");
                    }><icon::Github/></Button><myw::Gap w=8/>
            </div>
      <div style=" max-width: 400px; margin: auto; text-align: center">
        <div>
          <img
            style="max-width: 100px;filter: "
            src="img/nuoruo/nuoruo_logo_square.png"
            alt="nuoruo logo"
          />
        </div>
        <h2>"诺若浏览器"</h2>
        <p>"https://mayiwen.com/browser"</p>
        <myw::Gap h=30 />
          <Button style="transform: scale(1.2); transform-origin: center"  on_click=move |_| {
              open_url("https://gitlink.org.cn/mayiwen/nuoruo_browser/releases");
          }>前往下载</Button>
          <myw::Gap  h=12/>
          <a href="https://github.com/mayiwen/nuoruo_browser/releases" target="_blank">"前往github下载"</a>
          <myw::Gap h=12/>
        <div style="text-align: left;">
          <Tabset id=id>
            <Tab slot id=0 title="android".to_string()><android::I/></Tab>
            <Tab slot id=1 title="ios".to_string()><ios::I/></Tab>
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
