use crate::{
    models::{Login, SettingTab},
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
    let name: RwSignal<String> =
        RwSignal::new("mayiwen_in_the_development_environment".to_string());
    let pwd: RwSignal<String> = RwSignal::new("123456".to_string());
    let login = move |_| {
        spawn_local(async move {
            let name = name.get();
            let pwd = pwd.get();
            if name == "".to_string() || pwd == "".to_string() {
                let message: RwSignal<Vec<Message>> =
                    use_context::<RwSignal<Vec<Message>>>().expect("Message context must exist");

                let another_msg = Message {
                    t: myw::message::MessageType::ERROR,
                    m: "请输入内容".to_string(),
                };

                // 向 Vec 中添加新元素
                message.update(|msgs| {
                    msgs.push(another_msg); // push 是 Vec 的标准添加方法
                });
            } else {
                let res: Result<String, ServerFnError> = crate::login(name, pwd).await;
                match res {
                    Ok(res) => {
                        let message: RwSignal<Vec<Message>> =
                            use_context::<RwSignal<Vec<Message>>>()
                                .expect("Message context must exist");
                        let login = use_context::<RwSignal<Login>>()
                            .expect("Login context should be provided by parent component");
                        // 使用 update 方法修改 RwSignal 中的值
                        login.update(|login_state| {
                            login_state.token = res.to_string().clone();
                        });
                        crate::set_global_token(format!("{}", res.to_string())).unwrap();
                        // 打印更新后的 Token，确认是否写入成功
                        let new_token = crate::get_global_token();
                        eprintln!(
                            "[登录成功] set_global_token 调用成功，当前全局Token：{}",
                            new_token
                        );
                        let another_msg4 = Message {
                            t: myw::message::MessageType::INFO,
                            m: "马一文".to_string(),
                        };
                        let another_msg = Message {
                            t: myw::message::MessageType::INFO,
                            m: "欢迎归来".to_string(),
                        };
                        let another_msg2 = Message {
                            t: myw::message::MessageType::INFO,
                            m: "构建一流网站".to_string(),
                        };
                        let another_msg3 = Message {
                            t: myw::message::MessageType::INFO,
                            m: "勇攀技术巅峰".to_string(),
                        };
                        let another_msg5 = Message {
                            t: myw::message::MessageType::INFO,
                            m: new_token,
                        };
                        // 向 Vec 中添加新元素
                        message.update(|msgs| {
                            msgs.push(another_msg4); // push 是 Vec 的标准添加方法
                            msgs.push(another_msg); // push 是 Vec 的标准添加方法
                            msgs.push(another_msg2); // push 是 Vec 的标准添加方法
                            msgs.push(another_msg3); // push 是 Vec 的标准添加方法
                            msgs.push(another_msg5); // push 是 Vec 的标准添加方法
                        });
                        is_open.set(false)
                    }
                    Err(err) => {
                        let user_error_msg = format!("登录失败：{}", err); // 等价于 err.to_string()
                        let message: RwSignal<Vec<Message>> =
                            use_context::<RwSignal<Vec<Message>>>()
                                .expect("Message context must exist");

                        let another_msg = Message {
                            t: myw::message::MessageType::ERROR,
                            m: user_error_msg,
                        };

                        // 向 Vec 中添加新元素
                        message.update(|msgs| {
                            msgs.push(another_msg); // push 是 Vec 的标准添加方法
                        });
                    }
                };
            }
        });
    };

    view! {
        <Button  on_click=move |_| {
            is_open.set(true)
        }>管理登录</Button>
        <Modal is_open=is_open title=title  on_click=login >
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
