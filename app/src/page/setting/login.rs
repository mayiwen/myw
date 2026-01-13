use crate::{
    myw::{
        self,
        button::Button,
        modal::Modal,
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::prelude::*;

#[component]
pub fn I() -> impl IntoView {
    let is_open: RwSignal<bool> = RwSignal::new(false);
    let title: RwSignal<String> = RwSignal::new("管理登录".to_string());
    view! {
        <Button  on_click=move |_| {
            is_open.set(true)
        }>管理登录</Button>
        <Modal is_open=is_open title=title  on_click=move |_| {} >
            <div style="width: 350px; height: 200px">
            <div style="text-align: center">
            <myw::Gap h=40/>
            <input class="myw-input" placeholder="请输入用户名"/>
            <myw::Gap h=16/>
            <input class="myw-input" placeholder="请输入密码" />
            </div>

            </div>
        </Modal>
    }
}
