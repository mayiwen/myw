use crate::{
    myw::{
        self,
        message::Message,
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::prelude::*;
use myw::button::Button;
/// Renders the home page of your application.
#[component]
pub fn I() -> impl IntoView {
    let is_open: RwSignal<bool> = RwSignal::new(true);
    let title: RwSignal<String> = RwSignal::new("你好".to_string());
    view! {
        <myw::Gap/>
        <Button  on_click=move |_| {
            let message: RwSignal<Vec<Message>> =
                use_context::<RwSignal<Vec<Message>>>().expect("Message context must exist");
            let another_msg = Message {
                t: myw::message::MessageType::INFO,
                m: "默认消息",
            };
            // 向 Vec 中添加新元素
            message.update(|msgs| {
                msgs.push(another_msg); // push 是 Vec 的标准添加方法
            });

        }>默认消息</Button><myw::Gap w=8/>
        <Button  on_click=move |_| {
            let message: RwSignal<Vec<Message>> =
                use_context::<RwSignal<Vec<Message>>>().expect("Message context must exist");

            let another_msg = Message {
                t: myw::message::MessageType::ERROR,
                m: "错误消息",
            };

            // 向 Vec 中添加新元素
            message.update(|msgs| {
                msgs.push(another_msg); // push 是 Vec 的标准添加方法
            });

            }>错误消息</Button><myw::Gap w=8/>
        <Button  on_click=move |_| {
            let message: RwSignal<Vec<Message>> =
                use_context::<RwSignal<Vec<Message>>>().expect("Message context must exist");

            let another_msg = Message {
                t: myw::message::MessageType::WARNING,
                m: "警告消息",
            };
            // 向 Vec 中添加新元素
            message.update(|msgs| {
                msgs.push(another_msg); // push 是 Vec 的标准添加方法
            });

        }>警告消息</Button>

    }
}
