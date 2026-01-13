use std::{cell::RefCell, rc::Rc};

use leptos::{ev::MouseEvent, prelude::*};

use crate::myw;
use crate::myw::button::Button;
use crate::myw::icon;

#[component]
pub fn Modal(
    children: Children,
    /// 控制是否显示
    #[prop(optional, into)]
    is_open: RwSignal<bool>,
    #[prop(optional, into)] title: RwSignal<String>,
    // 按照官方文档风格添加点击事件prop（可选，FnMut(MouseEvent) + 'static） 点击确认调用的方法
    #[prop(optional)] on_click: Option<impl FnMut(MouseEvent) + 'static>,
) -> impl IntoView {
    // let border_style = move || {
    //     let border_value = border.get();
    //     if border_value == "both" {
    //         "1px solid var(--myw-border)".to_string()
    //     } else if border_value == "none" {
    //         "1px solid transparent".to_string()
    //     } else {
    //         "1px solid var(--myw-border)".to_string()
    //     }
    // };
    // // 手动处理空回调：有则执行，无则空操作
    // // 关键修复：用 Rc<RefCell<>> 包裹回调，保留多次调用的能力
    let on_click_rc = Rc::new(RefCell::new(on_click));
    let click_handler = move |e: MouseEvent| {
        // 借用而非转移所有权，支持多次调用
        if let Some(ref mut cb) = *on_click_rc.borrow_mut() {
            cb(e);
        }
    };

    view! {
        // <Show when=is_open>
        <div style:display=move || if is_open.get() {"block"} else {"none"} >
                <div
                    style="position: fixed; height: 100dvh; width: 100%; background-color: rgba(128, 128, 128, 0.5); z-index: 100000; left: 0; top: 0; opacity: 0.5;"
                    // 改动2：hidden 改为响应式闭包
                    hidden=move || !is_open.get()
                    on:click=move |_| is_open.set(false)
                ></div>
                <dialog
                open=move || is_open.get()
                    style="
                        border: 1px solid var(--myw-border);
                        background-color: var(--myw-bc);
                        border-radius: 4px;
                        opacity: 1;
                        position: fixed;
                        left: 50%;
                        top: 50%;
                        transform: translate(-50%, -50%);
                        z-index: 1000000;
                        background-color: var(--myw-bc);
                        color: var(--myw-color);
                    "
                    // on:click=|ev| ev.stop_propagation()
                >
                    // // 标题区域（可选）
                    <h3 style="padding-left: 4px; line-height: 40px;">
                        {title}
                        <Button
                            on_click=move |_| {is_open.set(false);}
                            style="float: right"
                            border="none"
                        >
                            <icon::Close />
                        </Button>

                    </h3>

                    // 内容区域
                    <div style="padding: 4px;">
                        {children()}
                    </div>

                    // // 按钮区域
                    <div style="float: right;">
                        <Button on_click=click_handler>
                            确认
                        </Button>

                        <myw::Gap w=8 />

                        <Button border="none" on_click=move |_| {
                            is_open.set(false)
                        }>
                            取消
                        </Button>
                    </div>
                </dialog>
            </div>


    }
}
