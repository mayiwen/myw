use crate::{
    myw::{
        self,
        button::Button,
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::prelude::*;
use leptos_router::components::Outlet;

/// Renders the home page of your application.
#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);
    view! {
      <myw::Gap h=8 />
      // "下方网站"
      // " "
      <div style=" max-width: 400px; margin: auto; text-align: center">
        <myw::Gap h=30 />
        <div>
          <img
            style="max-width: 150px;filter:  invert(35%) sepia(81%) saturate(2200%) hue-rotate(332deg) brightness(99%) contrast(101%)"
            src="img/nuoruo/nuoruo_logo_square.png"
            alt="nuoruo logo"
          />

        </div>
        <h2>"诺若浏览器主页"</h2>
        <p>"https://mayiwen.com/browser"</p>
        <p>"基于rust、wry、tauri、leptos构建"</p>
        <a href="https://github.com/mayiwen/nuoruo_browser">
          "https://github.com/mayiwen/nuoruo_browser"
        </a>
        <myw::Gap />
        // <h1>"nuoruo"</h1>
        // <h4>"诺若软件开发"</h4>
        // <p>"网站建设中"</p>
        <myw::Gap />
        // <p>"自用软件使用马一文的名义发布"</p>
        // <p>"通用软件使用诺若的名义发布"</p>
        // <myw::Gap />
        // <h4>"logo 演示"</h4>
        // <myw::Gap />

        // <h4>"域名持有"</h4>

        <myw::Gap h=30 />

        <myw::Gap h=8 />
        <img
          style="width: 100%; max-width: 200px;filter:  invert(35%) sepia(81%) saturate(2200%) hue-rotate(332deg) brightness(99%) contrast(101%);"
          src="img/nuoruo/nuoruo_logo.png"
          alt="nuoruo logo"
        />
        // <p>"nuoruo.com"</p>
        // <p>"nuoruo.cn"</p>
        // <p>"https://mayiwen.com/browser"</p>
        <p>"nuoruo网站暂未开放"</p>
        <p>"需访问mayiwen.com"</p>
        <myw::Gap h=30 />
      // <h4>"放这看"</h4>
      // <myw::Gap h=8/>
      // <p>"还没想好做什么"</p>
      // <p>"放在这里先看着"</p>
      // <p>"不定什么时候就有了灵感"</p>
      // <h4>"标语"</h4>
      // <myw::Gap h=8 />
      // <p>"构建一流网站"</p>
      // <p>"勇攀技术巅峰"</p>
      </div>
    }
}
