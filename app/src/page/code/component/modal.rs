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
use myw::button;
/// Renders the home page of your application.
#[component]
pub fn I() -> impl IntoView {
    let is_open: RwSignal<bool> = RwSignal::new(true);
    let title: RwSignal<String> = RwSignal::new("你好".to_string());
    view! {
        <myw::Gap h=8/>
        <Button  on_click=move |_| {
            is_open.set(true)
        }>打开弹窗</Button>
        <myw::Gap w=8/>
        <Modal is_open=is_open title=title  on_click=move |_| {} >
            <div style="width: 400px; height: 300px;">你好</div>
        </Modal>
    }
}
