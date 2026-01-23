use crate::myw::Gap;
use crate::page::home::logo::MayiwenLogo;
use leptos::prelude::*;
pub mod logo;
pub mod nav;
mod search;
use leptos::reactive::spawn_local;
#[component]
pub fn I() -> impl IntoView {
    // ✅ 2. 闭包中使用已获取的 navigate
    // // 1. 获取 App 组件提供的全局导航信号
    let global_nav =
        use_context::<RwSignal<Option<crate::NavFn>>>().expect("请确保 App 组件已提供全局导航信号");
    // 2. 关键修复：用 with() 替代 get()（无需 Clone）
    // let move_to_nuoruo =

    view! {
        <Gap h=30/>
        <MayiwenLogo/>
        <Gap h=30/>
        <search::I/>
        <Gap h=16/>
        <p style="text-align: center">
            <a
                style="text-decoration: underline; cursor: pointer;"
                title="一文小说阅读器，点击前往了解一下吧。"
                on:click= move |_| {
                    // with() 会借用内部值执行闭包，绕过 Clone 要求
                    global_nav.with(|nav_opt| {
                        if let Some(nav) = nav_opt {
                            nav("/yueduqi"); // 调用 App 封装的导航逻辑
                        };
                    });
                }
            >
                "前往一文小说阅读器"
            </a>
        </p>
        <Gap h=16/>
        <nav::I/>
    }
}
