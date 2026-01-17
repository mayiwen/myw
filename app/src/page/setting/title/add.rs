use std::sync::Arc;

use crate::models::title::Title;
use crate::{
    models::Login,
    myw::{
        self,
        button::Button,
        message::Message,
        table::{TabColumn, Table},
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::{prelude::*, reactive::spawn_local};
#[component]
pub fn I() -> impl IntoView {
    let title: RwSignal<String> = RwSignal::new("你好".to_string());
    let login = move |_| {
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
                    }
                    // WrappedServerError(E),
                    // /// Error while trying to register the server function (only occurs in case of poisoned RwLock).
                    // Registration(String),
                    // /// Occurs on the client if there is a network error while trying to run function on server.
                    // Request(String),
                    // /// Occurs on the server if there is an error creating an HTTP response.
                    // Response(String),
                    // /// Occurs when there is an error while actually running the function on the server.
                    // ServerError(String),
                    // /// Occurs when there is an error while actually running the middleware on the server.
                    // MiddlewareError(String),
                    // /// Occurs on the client if there is an error deserializing the server's response.
                    // Deserialization(String),
                    // /// Occurs on the client if there is an error serializing the server function arguments.
                    // Serialization(String),
                    // /// Occurs on the server if there is an error deserializing one of the arguments that's been sent.
                    // Args(String),
                    // /// Occurs on the server if there's a missing argument.
                    // MissingArg(String),
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
