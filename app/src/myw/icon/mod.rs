use leptos::prelude::*;

#[component]
pub fn Myw() -> impl IntoView {
    view! {
        <img
            height="32px"
            width="32px"
            style="vertical-align: middle; transform: translateY(-2px);"
            src="data:image/x-icon;base64,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"
        />
    }
}
use leptos::*;

/// 搜索图标组件
#[component]
pub fn Search(
    /// 图标宽度和高度（像素）
    #[prop(default = 24)]
    wh: usize,
    /// 自定义样式
    #[prop(default = "".to_string())]
    style: String,
) -> impl IntoView {
    // 计算尺寸字符串
    let size_str = format!("{}px", wh);
    // 组合样式（基础样式 + 自定义样式）
    let combined_style = format!("vertical-align: middle; {style}");

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            view_box="0 0 24 24"
            fill="var(--myw-color)"
            width=size_str.clone()
            height=size_str
            style=combined_style
        >
            <path d="M18.031 16.6168L22.3137 20.8995L20.8995 22.3137L16.6168 18.031C15.0769 19.263 13.124 20 11 20C6.032 20 2 15.968 2 11C2 6.032 6.032 2 11 2C15.968 2 20 6.032 20 11C20 13.124 19.263 15.0769 18.031 16.6168ZM16.0247 15.8748C17.2475 14.6146 18 12.8956 18 11C18 7.1325 14.8675 4 11 4C7.1325 4 4 7.1325 4 11C4 14.8675 7.1325 18 11 18C12.8956 18 14.6146 17.2475 15.8748 16.0247L16.0247 15.8748Z" />
        </svg>
    }
}
