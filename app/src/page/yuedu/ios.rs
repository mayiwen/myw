use crate::myw;
use leptos::prelude::*;

#[component]
pub fn I() -> impl IntoView {
    view! {
        <div style=" max-width: 400px; margin: auto;">
            <myw::Gap/>
            <h3>"ios 主界面"</h3>
            <myw::Gap/>
            <img style="width: 100%; max-width: 400px;" src="img/nuoruo/yueduqi/yueduqi_ios.jpg" alt="ios image" />
            <h4>"截图来源"</h4>
            <p>"系统：ios 26"</p>
            <p>"CPU架构：Apple"</p>
            <p>"设备信息：iPhone 17 pro max"</p>
            <myw::Gap/>
            <h4>"发布日期"</h4>
            <p>"2025-11-23"</p>
        </div>
    }
}
