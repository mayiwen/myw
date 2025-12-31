use leptos::{prelude::*, task::spawn_local};

use crate::greet;

#[component]
pub fn I() -> impl IntoView {
    // 响应式信号：存储服务端返回的字符串
    let show_string = RwSignal::new("".to_string());

    view! {
        <button
            on:click=move |_| {
                // 点击事件：异步调用服务端函数
                spawn_local(async move {
                    // 修复点1：处理 ServerFnError 并给出友好提示
                    let result = crate::get_title().await;
                    let a = match result {
                        Ok(s) => s,
                        Err(e) => format!("请求失败: {}", e), // 错误信息格式化
                    };
                    // 修复点2：正确更新响应式信号
                    show_string.set(a);
                });
            }
        >
            "Add Todo"
        </button>
        // 修复点3：响应式信号渲染（直接用 {show_string} 也可以，但推荐用 move 确保捕获）
        <p>{move || show_string.get()}</p>
    }
}
