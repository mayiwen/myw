use std::{cell::RefCell, rc::Rc};

use gloo_timers::callback::Interval;
use leptos::reactive::spawn_local;
use leptos::{ev::MouseEvent, prelude::*};

use crate::myw;
use crate::myw::button::Button;
use crate::myw::icon;
#[derive(Clone, PartialEq)]
pub enum MessageType {
    WARNING,
    ERROR,
    INFO,
}
#[derive(Clone, PartialEq)]
pub struct Message {
    /// type
    pub t: MessageType,
    /// message
    pub m: &'static str,
}
use gloo_timers::future::TimeoutFuture; // 异步定时器（替代同步 Interval）
#[component]
pub fn MessageCreate() -> impl IntoView {
    // 1. 获取全局消息信号（保留原有逻辑）
    let message: RwSignal<Vec<Message>> =
        use_context::<RwSignal<Vec<Message>>>().expect("Message context must exist");

    // 2. 初始化定时逻辑的状态（替代原 Rc<RefCell> 写法，保持逻辑一致）
    let flag = Rc::new(RefCell::new(false));

    // 3. 核心优化：用 create_effect + 异步循环实现定时逻辑（支持 async，替代原 Resource 方案）
    Effect::new(move |_| {
        let message_clone = message.clone();
        let flag_clone = Rc::clone(&flag);

        // 启动客户端异步任务（Leptos 0.8 客户端异步必须用 spawn_local）
        spawn_local(async move {
            // 无限循环模拟 setInterval，支持 async 逻辑，组件销毁时自动停止
            loop {
                // --------------------------
                // 原有核心移除逻辑（完全保留，仅适配异步上下文）
                // --------------------------
                let mut flag = flag_clone.borrow_mut();
                let mut msgs = message_clone.get();

                if *flag {
                    if msgs.first().is_some() {
                        msgs.remove(0);
                        *flag = false;
                    }
                } else {
                    if msgs.len() == 1 {
                        *flag = true;
                    } else if msgs.first().is_some() {
                        msgs.remove(0);
                    }
                }

                // 更新信号触发 UI 刷新
                message_clone.set(msgs);

                // --------------------------
                // 关键：等待 1500ms（替代同步 Interval，支持 async 暂停）
                // --------------------------
                TimeoutFuture::new(1500).await;

                // // 可选：添加日志验证执行（方便调试，可删除）
                // web_sys::console::log_1(
                //     &format!("定时执行，剩余消息数：{}", message_clone.get().len()).into(),
                // );
            }
        });
    });

    // 响应式渲染消息列表
    view! {
        <div style="position: fixed; left: 50%; top: 20px; transform: translate(-50%, 0); z-index: 100000000000000;">
              {move || {
                  message.get().into_iter()
                      .map(|n| view! {
                          <p
                              style="
                                  border: 1px solid var(--myw-border);
                                  border-radius: 4px;
                                  padding: 4px;
                                  color: var(--myw-bc);
                                  margin-bottom: 4px;
                                  min-width: 300px;
                                  width: 100%;
                                  max-width: 800px;
                                  opacity: 0.9;"
                              style:background-color=move || match n.t {
                                  MessageType::WARNING => "var(--myw-yellowDefault)",
                                  MessageType::ERROR => "var(--myw-redDefault)",
                                  MessageType::INFO => "var(--myw-blueDefault)",
                              }
                          >
                              {n.m.clone()}
                          </p>
                      })
                      .collect::<Vec<_>>()
              }}
          </div>
    }
}
