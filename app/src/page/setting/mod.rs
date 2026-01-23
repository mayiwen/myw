use crate::{
    models::{Login, SettingTab},
    myw::{
        self,
        button::Button,
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::prelude::*;
pub mod about;
pub mod link;
pub mod login;
pub mod title;
#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);
    let login = use_context::<RwSignal<Login>>()
        .expect("Login context should be provided by parent component");
    // 关键修复：创建响应式的取值闭包，让 Leptos 追踪状态变化
    // 闭包会在每次状态变化时重新执行，触发组件重新渲染
    let current_token = move || login.with(|login_state| login_state.token.clone());

    Effect::new(move |_| {
        // 获取到
        let tab: Option<RwSignal<SettingTab>> = use_context::<RwSignal<SettingTab>>();
        match tab {
            Some(v) => {
                if v.get().value == 2 {
                    id.set(2);
                    v.update(|st| {
                        st.value = 0;
                    });
                }
            }
            None => {}
        }
    });
    view! {
        <myw::Gap/>
        <div style="float: right;">
            <login::I/>
        </div>
        <h1>设置与关于</h1>
        <div style="text-align: center; word-break: break-all;" class="">   {current_token}</div>
        <myw::Gap/>
        <Tabset id=id>
            <Tab slot id=0 title="首页标题".to_string()><title::I/></Tab>
            <Tab slot id=1 title="首页链接".to_string()><link::I/></Tab>
            <Tab slot id=2 title="关于".to_string()><about::I/></Tab>
        </Tabset>
    }
}
