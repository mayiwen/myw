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

#[component]
pub fn Theme(
    /// 图标宽度和高度（像素）
    #[prop(default = 24)]
    wh: usize,
    #[prop(default = String::new())] style: String,
) -> impl IntoView {
    let size_str = format!("{}px", wh);
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            view_box="0 0 24 24"
            fill="var(--myw-color)"
            width=size_str.clone()
            height=size_str
            style=format!("vertical-align: middle; {style}")
        >
            <path d="M15.1986 9.94447C14.7649 9.5337 14.4859 8.98613 14.4085 8.39384L14.0056 5.31138L11.275 6.79724C10.7503 7.08274 10.1433 7.17888 9.55608 7.06948L6.49998 6.50015L7.06931 9.55625C7.17871 10.1435 7.08257 10.7505 6.79707 11.2751L5.31121 14.0057L8.39367 14.4086C8.98596 14.4861 9.53353 14.7651 9.94431 15.1987L12.0821 17.4557L13.4178 14.6486C13.6745 14.1092 14.109 13.6747 14.6484 13.418L17.4555 12.0823L15.1986 9.94447ZM15.2238 15.5079L13.0111 20.1581C12.8687 20.4573 12.5107 20.5844 12.2115 20.442C12.1448 20.4103 12.0845 20.3665 12.0337 20.3129L8.49229 16.5741C8.39749 16.474 8.27113 16.4096 8.13445 16.3918L3.02816 15.7243C2.69958 15.6814 2.46804 15.3802 2.51099 15.0516C2.52056 14.9784 2.54359 14.9075 2.5789 14.8426L5.04031 10.3192C5.1062 10.1981 5.12839 10.058 5.10314 9.92253L4.16 4.85991C4.09931 4.53414 4.3142 4.22086 4.63997 4.16017C4.7126 4.14664 4.78711 4.14664 4.85974 4.16017L9.92237 5.10331C10.0579 5.12855 10.198 5.10637 10.319 5.04048L14.8424 2.57907C15.1335 2.42068 15.4979 2.52825 15.6562 2.81931C15.6916 2.88421 15.7146 2.95507 15.7241 3.02833L16.3916 8.13462C16.4095 8.2713 16.4739 8.39766 16.5739 8.49245L20.3127 12.0338C20.5533 12.2617 20.5636 12.6415 20.3357 12.8821C20.2849 12.9357 20.2246 12.9795 20.1579 13.0112L15.5078 15.224C15.3833 15.2832 15.283 15.3835 15.2238 15.5079ZM16.0206 17.435L17.4348 16.0208L21.6775 20.2634L20.2633 21.6776L16.0206 17.435Z"/>
        </svg>
    }
}

#[component]
pub fn Github(
    /// 图标宽度和高度（像素）
    #[prop(default = 24)]
    wh: usize,
    #[prop(default = String::new())] style: String,
) -> impl IntoView {
    let size_str = format!("{}px", wh);
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            view_box="0 0 24 24"
            fill="var(--myw-color)"
            width=size_str.clone()
            height=size_str
            style=format!("vertical-align: middle; {style}")
        >
            <path d="M12.001 2C6.47598 2 2.00098 6.475 2.00098 12C2.00098 16.425 4.86348 20.1625 8.83848 21.4875C9.33848 21.575 9.52598 21.275 9.52598 21.0125C9.52598 20.775 9.51348 19.9875 9.51348 19.15C7.00098 19.6125 6.35098 18.5375 6.15098 17.975C6.03848 17.6875 5.55098 16.8 5.12598 16.5625C4.77598 16.375 4.27598 15.9125 5.11348 15.9C5.90098 15.8875 6.46348 16.625 6.65098 16.925C7.55098 18.4375 8.98848 18.0125 9.56348 17.75C9.65098 17.1 9.91348 16.6625 10.201 16.4125C7.97598 16.1625 5.65098 15.3 5.65098 11.475C5.65098 10.3875 6.03848 9.4875 6.67598 8.7875C6.57598 8.5375 6.22598 7.5125 6.77598 6.1375C6.77598 6.1375 7.61348 5.875 9.52598 7.1625C10.326 6.9375 11.176 6.825 12.026 6.825C12.876 6.825 13.726 6.9375 14.526 7.1625C16.4385 5.8625 17.276 6.1375 17.276 6.1375C17.826 7.5125 17.476 8.5375 17.376 8.7875C18.0135 9.4875 18.401 10.375 18.401 11.475C18.401 15.3125 16.0635 16.1625 13.8385 16.4125C14.201 16.725 14.5135 17.325 14.5135 18.2625C14.5135 19.6 14.501 20.675 14.501 21.0125C14.501 21.275 14.6885 21.5875 15.1885 21.4875C19.259 20.1133 21.9999 16.2963 22.001 12C22.001 6.475 17.526 2 12.001 2Z"/>
        </svg>
    }
}
#[component]
pub fn Bookmark(
    /// 图标宽度和高度（像素）
    #[prop(default = 24)]
    wh: usize,
    #[prop(default = String::new())] style: String,
) -> impl IntoView {
    let size_str = format!("{}px", wh);
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            view_box="0 0 24 24"
            fill="var(--myw-color)"
            width=size_str.clone()
            height=size_str
            style=format!("vertical-align: middle; {style}")
        >
            <path d="M24 12L18.3431 17.6569L16.9289 16.2426L21.1716 12L16.9289 7.75736L18.3431 6.34315L24 12ZM2.82843 12L7.07107 16.2426L5.65685 17.6569L0 12L5.65685 6.34315L7.07107 7.75736L2.82843 12ZM9.78845 21H7.66009L14.2116 3H16.3399L9.78845 21Z"/>
        </svg>
    }
}

#[component]
pub fn GitLink(
    /// 图标宽度和高度（像素）
    #[prop(default = 24)]
    wh: usize,
    #[prop(default = String::new())] style: String,
) -> impl IntoView {
    let size_str = format!("{}px", wh);
    view! {
        <svg
            version="1.1"
            id="Layer_1"
            xmlns="http://www.w3.org/2000/svg"
            x="0px"
            y="0px"
            width=size_str.clone()
            height=size_str
            view_box="0 0 24 24"
            style=format!("vertical-align: middle; {style}")
        >
            <image
                id="image0"
                width="24"
                height="24"
                x="0"
                y="0"
                href="data:image/png;base64,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"
            />
        </svg>
    }
}

#[component]
pub fn Close(
    /// 图标宽度和高度（像素）
    #[prop(default = 40)]
    wh: usize,
    #[prop(default = "".to_string())] style: String,
) -> impl IntoView {
    let size_str = format!("{}px", wh);
    let base_style = format!(
        "display: inline-flex; align-items: center; justify-content: center; vertical-align: middle; {style}"
    );

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            view_box="0 0 40 40"
            fill="var(--myw-color)"
            width=size_str.clone()
            height=size_str
            style=base_style
        >
            // 核心修改：给 path 加平移，让 13x13 的图形在 40x40 画布居中
            <path
                d="M11.9997 10.5865L16.9495 5.63672L18.3637 7.05093L13.4139 12.0007L18.3637 16.9504L16.9495 18.3646L11.9997 13.4149L7.04996 18.3646L5.63574 16.9504L10.5855 12.0007L5.63574 7.05093L7.04996 5.63672L11.9997 10.5865Z"
                // 40-13=27，27/2=13.5，平移 13.5 让图形居中
                style="transform:translate(7.5px, 7.5px)"
            />
        </svg>
    }
}
