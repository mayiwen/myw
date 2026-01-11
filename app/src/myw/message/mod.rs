use std::{cell::RefCell, rc::Rc};

use gloo_timers::callback::Interval;
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

#[component]
pub fn MessageCreate() -> impl IntoView {
    let message: RwSignal<Vec<Message>> =
        use_context::<RwSignal<Vec<Message>>>().expect("to have found the setter provided");
    let flag = Rc::new(RefCell::new(false));
    Effect::new(move |_| {
        let flag_clone = Rc::clone(&flag);

        let interval = Interval::new(1500, move || {
            // 关键3：仅借用 flag，不移动，确保闭包可多次执行
            let mut flag = flag_clone.borrow_mut();
            let mut msgs = message.get();

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

            // message.set(msgs);
            untrack(move || message.set(msgs));
        });
    });
    view! {
        <div style="position: fixed; left: 50%; top: 20px; transform: translate(-50%, 0); z-index: 100000000000000;">
              // 核心修复：用 move || {} 闭包包裹渲染逻辑，实现响应式监听
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
                              // 修复：移除多余的 format!，直接返回字符串
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
