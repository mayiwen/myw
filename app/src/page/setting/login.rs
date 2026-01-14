use crate::{
    myw::{
        self,
        button::Button,
        message::Message,
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
    let name: RwSignal<String> = RwSignal::new("".to_string());
    let pwd: RwSignal<String> = RwSignal::new("".to_string());
    let login = move |_| {
        spawn_local(async move {
            let name = name.get();
            let pwd = pwd.get();
            if name == "".to_string() || pwd == "".to_string() {
                let message: RwSignal<Vec<Message>> =
                    use_context::<RwSignal<Vec<Message>>>().expect("Message context must exist");

                let another_msg = Message {
                    t: myw::message::MessageType::ERROR,
                    m: "请输入内容",
                };

                // 向 Vec 中添加新元素
                message.update(|msgs| {
                    msgs.push(another_msg); // push 是 Vec 的标准添加方法
                });
            }
            let res = crate::login(name, pwd).await;
            match res {
                Ok(res) => {
                    str.set(res);
                    let message: RwSignal<Vec<Message>> = use_context::<RwSignal<Vec<Message>>>()
                        .expect("Message context must exist");

                    let another_msg4 = Message {
                        t: myw::message::MessageType::INFO,
                        m: "马一文",
                    };
                    let another_msg = Message {
                        t: myw::message::MessageType::INFO,
                        m: "欢迎归来",
                    };
                    let another_msg2 = Message {
                        t: myw::message::MessageType::INFO,
                        m: "构建一流网站",
                    };
                    let another_msg3 = Message {
                        t: myw::message::MessageType::INFO,
                        m: "勇攀技术巅峰",
                    };
                    // 向 Vec 中添加新元素
                    message.update(|msgs| {
                        msgs.push(another_msg4); // push 是 Vec 的标准添加方法
                        msgs.push(another_msg); // push 是 Vec 的标准添加方法
                        msgs.push(another_msg2); // push 是 Vec 的标准添加方法
                        msgs.push(another_msg3); // push 是 Vec 的标准添加方法
                    });
                    is_open.set(false)
                }
                Err(_) => {
                    let message: RwSignal<Vec<Message>> = use_context::<RwSignal<Vec<Message>>>()
                        .expect("Message context must exist");

                    let another_msg = Message {
                        t: myw::message::MessageType::ERROR,
                        m: "马一文才可以登录",
                    };

                    // 向 Vec 中添加新元素
                    message.update(|msgs| {
                        msgs.push(another_msg); // push 是 Vec 的标准添加方法
                    });
                }
            };
        });
    };
    view! {
        <Button  on_click=move |_| {
            is_open.set(true)
        }>管理登录</Button>
        <Modal is_open=is_open title=title  on_click=login >

            // <div style="text-align: center; word-break: break-all;" class="">   {str}</div>
            <div style="width: 350px; height: 200px">
            <div style="text-align: center">

            <myw::Gap h=40/>
            <input class="myw-input" placeholder="请输入用户名" bind:value=name/>
            <myw::Gap h=16/>
            <input class="myw-input" placeholder="请输入密码" bind:value=pwd/>
               <myw::Gap h=16/>
            // <Button  on_click=login style="width: 100px;">登录</Button>
            </div>

            </div>
        </Modal>
    }
}
