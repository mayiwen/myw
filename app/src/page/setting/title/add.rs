use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::{
    myw::{
        self,
        button::Button,
        message::Message,
        table::{TabColumn, Table},
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::{ev::MouseEvent, prelude::*, reactive::spawn_local};
#[component]
pub fn I(#[prop(optional)] on_click: Option<impl FnMut(bool) + 'static>) -> impl IntoView {
    // ========== 关键修改1：替换 Arc 为 Rc（单线程场景无需 Arc） ==========
    let on_click_rc = Rc::new(RefCell::new(on_click));
    let title: RwSignal<String> = RwSignal::new("你好".to_string());
    let login = move |_| {
        // ========== 关键修改2：克隆 Rc 到异步闭包（避免所有权转移导致消耗） ==========
        let on_click_rc_clone = on_click_rc.clone();
        let title_clone = title.clone(); // 克隆信号，避免 move 消耗
        spawn_local(async move {
            let title = title.get();
            if title.is_empty() {
                let message: RwSignal<Vec<Message>> =
                    use_context::<RwSignal<Vec<Message>>>().expect("Message context must exist");
                let another_msg = Message {
                    t: myw::message::MessageType::ERROR,
                    m: "空数据".to_string(),
                };
                // 向 Vec 中添加新元素
                message.update(|msgs| {
                    msgs.push(another_msg); // push 是 Vec 的标准添加方法
                });
            } else {
                let token = crate::get_global_token_with_bearer();
                let res = crate::create_title(title, token).await;

                match res {
                    Ok(res) => {
                        let message: RwSignal<Vec<Message>> =
                            use_context::<RwSignal<Vec<Message>>>()
                                .expect("Message context must exist");
                        // 使用 update 方法修改 RwSignal 中的值
                        let another_msg4 = Message {
                            t: myw::message::MessageType::INFO,
                            m: "添加成功".to_string(),
                        };
                        // 向 Vec 中添加新元素
                        message.update(|msgs| {
                            msgs.push(another_msg4); // push 是 Vec 的标准添加方法
                        });

                        // ========== 关键修改3：正确处理 FnMut 闭包的调用（避免消耗） ==========
                        // 1. 短生命周期借用 RefCell（调用后立即释放）
                        // 2. 用 if let 匹配后直接调用，不持有长期引用
                        if let Ok(mut cb_opt) = on_click_rc_clone.try_borrow_mut() {
                            if let Some(cb) = &mut *cb_opt {
                                cb(true); // 调用 FnMut 闭包，支持多次调用
                            }
                        }
                    }
                    Err(res) => {
                        let error_msg = match res {
                            ServerFnError::ServerError(msg) => msg, // 提取自定义服务端错误信息
                            ServerFnError::Registration(msg) => msg, // 提取自定义服务端错误信息
                            ServerFnError::Request(msg) => msg,     // 提取自定义服务端错误信息
                            ServerFnError::Response(msg) => msg,    // 提取自定义服务端错误信息
                            ServerFnError::MiddlewareError(msg) => msg, // 提取自定义服务端错误信息
                            ServerFnError::Deserialization(msg) => msg, // 提取自定义服务端错误信息
                            ServerFnError::Serialization(msg) => msg, // 提取自定义服务端错误信息
                            ServerFnError::Args(msg) => msg,        // 提取自定义服务端错误信息
                            ServerFnError::MissingArg(msg) => msg,  // 提取自定义服务端错误信息
                            ServerFnError::WrappedServerError(msg) => {
                                "WrappedServerError".to_string()
                            } // 提取自定义服务端错误信息
                            _ => "添加失败1".to_string(),           // 匹配所有其他未知错误
                        };
                        let message: RwSignal<Vec<Message>> =
                            use_context::<RwSignal<Vec<Message>>>()
                                .expect("Message context must exist");

                        let another_msg = Message {
                            t: myw::message::MessageType::ERROR,
                            m: error_msg,
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
        <myw::Gap/>
        <input class="myw-input" placeholder="请输入标题" bind:value=title/> <myw::Gap w=8/>
        <Button  on_click=login >添加</Button>
    }
}
