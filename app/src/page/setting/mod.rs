use crate::{
    myw::{
        self,
        button::Button,
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::prelude::*;
pub mod about;
pub mod title;
#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);
    view! {
        <myw::Gap/>
        <h1>设置与关于</h1>
            <div style="float: right;">
            <Button on_click=move |_| {}>"管理登录"</Button>
            </div>
        <myw::Gap/>
        <Tabset id=id>
            <Tab slot id=0 title="首页标题".to_string()><title::I/></Tab>
            <Tab slot id=1 title="首页链接".to_string()>""</Tab>
            <Tab slot id=2 title="关于".to_string()><about::I/></Tab>
        </Tabset>
    }
}
