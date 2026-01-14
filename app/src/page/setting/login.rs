use crate::{
    myw::{
        self,
        button::Button,
        modal::Modal,
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::{prelude::*, reactive::spawn_local};

#[component]
pub fn I() -> impl IntoView {
    let is_open: RwSignal<bool> = RwSignal::new(false);
    let str: RwSignal<String> = RwSignal::new("".to_string());
    let title: RwSignal<String> = RwSignal::new("管理登录".to_string());

    Effect::new(move |_| {
        spawn_local(async move {
            let res = crate::login("mayiwen".to_string(), "MA@yiwen".to_string()).await;
            match res {
                Ok(res) => {
                    str.set(res);
                }
                Err(_) => {}
            }
        });
    });
    view! {
        <Button  on_click=move |_| {
            is_open.set(true)
        }>管理登录</Button>
        <Modal is_open=is_open title=title  on_click=move |_| {} >

                <div style="text-align: center; word-break: break-all;" class="">   {str}</div>
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
