use std::{cell::RefCell, rc::Rc};

use leptos::{ev::MouseEvent, prelude::*};

#[component]
pub fn I(
    children: Children,
    #[prop(optional, into)] border: Signal<String>,
    #[prop(optional, into)] style: Signal<String>,
    #[prop(optional, into)] active: Signal<bool>,
) -> impl IntoView {
    let border_style = move || {
        let border_value = border.get();
        if border_value == "both" {
            "1px solid var(--myw-border)".to_string()
        } else if border_value == "none" {
            "1px solid transparent".to_string()
        } else {
            "1px solid var(--myw-border)".to_string()
        }
    };

    view! {
        <button
            class="myw-button"
            style:border=border_style
            style=style
            class:active=move ||active.get()

            >{children()}
        </button>
    }
}

#[component]
pub fn Button(
    children: Children,
    #[prop(optional, into)] border: Signal<String>,
    #[prop(optional, into)] style: Signal<String>,
    #[prop(optional, into)] active: Signal<bool>,
    // 按照官方文档风格添加点击事件prop（可选，FnMut(MouseEvent) + 'static）
    #[prop(optional)] on_click: Option<impl FnMut(MouseEvent) + 'static>,
) -> impl IntoView {
    let border_style = move || {
        let border_value = border.get();
        if border_value == "both" {
            "1px solid var(--myw-border)".to_string()
        } else if border_value == "none" {
            "1px solid transparent".to_string()
        } else {
            "1px solid var(--myw-border)".to_string()
        }
    };
    // 手动处理空回调：有则执行，无则空操作
    // 关键修复：用 Rc<RefCell<>> 包裹回调，保留多次调用的能力
    let on_click_rc = Rc::new(RefCell::new(on_click));
    let click_handler = move |e: MouseEvent| {
        // 借用而非转移所有权，支持多次调用
        if let Some(ref mut cb) = *on_click_rc.borrow_mut() {
            cb(e);
        }
    };

    view! {
        <button
            class="myw-button"
            style:border=border_style
            style=style
            class:active=move || active.get()
            // 绑定点击事件，和官方文档写法一致
            on:click=click_handler
        >
            {children()}
        </button>
    }
}
