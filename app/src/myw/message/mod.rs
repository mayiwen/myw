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
    pub m: String,
}
use gloo_timers::future::TimeoutFuture;
#[component]
pub fn MessageCreate() -> impl IntoView {
    let message = use_context::<RwSignal<Vec<Message>>>().expect("Message context must exist");
    let state = Rc::new(RefCell::new((false, 0, false)));
    Effect::new(move |_| {
        let message_clone = message.clone();
        let state_clone = Rc::clone(&state);
        spawn_local(async move {
            loop {
                let mut state = state_clone.borrow_mut();
                let (flag, prev_len, force_remove) = &mut *state;
                let mut msgs = message_clone.get();
                let curr_len = msgs.len();
                if curr_len > *prev_len {
                    *force_remove = true;
                    *prev_len = curr_len;
                } else {
                    if *force_remove {
                        if msgs.first().is_some() {
                            msgs.remove(0);
                        }
                        *force_remove = false;
                    } else {
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
                    }
                    *prev_len = curr_len;
                }
                // ========== 新增：限制最多显示5条消息 ==========
                // 保留最后5条（最新的5条），移除超出的旧消息
                if msgs.len() > 5 {
                    // 计算需要保留的起始索引，截断前面的旧消息
                    let keep_start = msgs.len() - 5;
                    msgs.drain(0..keep_start); // 原地删除超出的旧消息，不移动msgs所有权
                };
                // =============================================
                message_clone.set(msgs);
                drop(state);
                TimeoutFuture::new(1500).await;
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
