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
pub fn I(
    #[prop(optional)] on_click: Option<impl FnMut(bool) + 'static>,
    id: RwSignal<u64>,
) -> impl IntoView {
    let on_click_rc = Rc::new(RefCell::new(on_click));
    let title: RwSignal<String> = RwSignal::new("".to_string());
    let src: RwSignal<String> = RwSignal::new("".to_string());
    let id_str = RwSignal::new("0".to_string());
    Effect::new(move |_| {
        let current_id = id.get(); // 访问 id 信号，建立依赖
        let new_id_str = current_id.to_string();
        id_str.set(new_id_str); // 可读写信号的 set 会触发 DOM 重渲染
    });
    let login = move |_| {
        let on_click_rc_clone = on_click_rc.clone();
        let title_clone = title.clone(); // 克隆信号，避免 move 消耗
        let src_clone = src.clone(); // 克隆信号，避免 move 消耗
        spawn_local(async move {
            let title = title.get();
            let src = src.get();
            if title.is_empty() || src.is_empty() {
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
                let res =
                    crate::create_link(id.get(), title_clone.get(), src_clone.get(), token).await;

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
        // {id}
        <input class="myw-input" placeholder="请输入id" value=id_str disabled/> <myw::Gap w=8/>
        <input class="myw-input" placeholder="请输入标题" bind:value=title/> <myw::Gap w=8/>
        <input class="myw-input" placeholder="请输入链接" bind:value=src/> <myw::Gap w=8/>
        <Button  on_click=login >添加</Button>
          <myw::Gap/>
    }
}
