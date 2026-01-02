use crate::myw::button::Button;
use crate::myw::icon::Search;
use crate::myw::Gap;
use crate::util::open_url;
use leptos::{ev, prelude::*};
#[component]
pub fn I() -> impl IntoView {
    // 响应式状态：搜索文本（对应原代码中的 text）
    let text = RwSignal::new(String::new());
    // 处理键盘按下事件（Enter 搜索）
    let handle_keydown = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Enter" {
            ev.prevent_default();
            let search_text = text.get();
            let url = format!("https://www.baidu.com/s?wd={}", search_text);
            open_url(&url);
        }
    };

    view! {
        <div style="width: 100%; text-align: center;">
            <div style="margin: auto; max-width: 600px; width: 100%; position: relative;">
                <input
                    type="text"
                    prop:value=text
                    class="myw-input"
                    style="height: 40px;max-width: 600px; width: 100%; padding-right: 60px;"
                    placeholder="请输入搜索的内容，键入Enter调用百度搜索"
                    on:input=move |ev| {
                        text.set(event_target_value(&ev));
                    }
                    on:keydown=handle_keydown
                />
                <Button
                    style="position: absolute; right:1px; top: 1px; width: 50px;"
                    border="none"
                    on_click=move |_| {
                        let search_text = text.get();
                        let url = format!("https://www.baidu.com/s?wd={}", search_text);
                        open_url(&url);
                    }
                >
                    <Search />
                </Button>
            </div>
        </div>
    }
}
