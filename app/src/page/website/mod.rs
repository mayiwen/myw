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
        // "旗下网站"
        " "
        <div style=" max-width: 400px; margin: auto; text-align: center">
            <myw::Gap/>
            <h1>"nuoruo"</h1>
            <myw::Gap/>
            <h4>"logo 演示"</h4>
            <myw::Gap/>
            <img style="width: 100%; max-width: 400px;" src="img/nuoruo/logo.png" alt="ios image" />
            <myw::Gap h=30/>
            <h4>"域名持有"</h4>
             <myw::Gap h=8/>
            <p>"nuoruo.com"</p>
            <p>"nuoruo.cn"</p>
            <myw::Gap h=30/>
            // <h4>"放这看"</h4>
            // <myw::Gap h=8/>
            // <p>"还没想好做什么"</p>
            // <p>"放在这里先看着"</p>
            // <p>"不定什么时候就有了灵感"</p>
            // <h4>"标语"</h4>
            //     <myw::Gap h=8/>
            // <p>"构建一流网站"</p>
            // <p>"勇攀技术巅峰"</p>
        </div>


    }
}
