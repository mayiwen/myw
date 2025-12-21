use crate::myw;
use leptos::prelude::*;

#[component]
pub fn I() -> impl IntoView {
    let arr_vec = vec![
        Img::new("主界面", "img/nuoruo/yueduqi/yueduqi_windows.png"),
        Img::new("阅读界面", "img/nuoruo/yueduqi/feature/read.png"),
        Img::new("设置界面", "img/nuoruo/yueduqi/feature/setting.png"),
        Img::new("黑色主题", "img/nuoruo/yueduqi/feature/black.png"),
        Img::new("自定义背景", "img/nuoruo/yueduqi/feature/bg.png"),
        Img::new("获取目录", "img/nuoruo/yueduqi/feature/ch.png"),
        Img::new(
            "拖拽右下角改变为任意大小 仅桌面端支持",
            "img/nuoruo/yueduqi/feature/drag.png",
        ),
    ];

    view! {
        <div style=" max-width: 400px; margin: auto;">
            <For
                each=move || arr_vec.clone()
                key=|state| state.title.clone()
                let:child
            >
                <myw::Gap/>
                <h3>{child.title}</h3>
                <myw::Gap h=8/>
                <img style="width: 100%; max-width: 400px;" src=child.src/>
            </For>
        </div>
    }
}
#[derive(Debug, Clone, Copy)]
struct Img {
    pub title: &'static str,
    pub src: &'static str,
}
impl Img {
    fn new(title: &'static str, src: &'static str) -> Self {
        Self { title, src }
    }
}
