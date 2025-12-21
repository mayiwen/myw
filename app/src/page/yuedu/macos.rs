use crate::myw;
use leptos::prelude::*;

#[component]
pub fn I() -> impl IntoView {
    view! {
        <div style=" max-width: 400px; margin: auto;">
            <myw::Gap/>
            <h3>"macos 主界面"</h3>
            <myw::Gap/>
            <img style="width: 100%; max-width: 400px;" src="img/nuoruo/yueduqi/yueduqi_dmg.png" alt="ios image" />
            <h4>"截图来源"</h4>
            <p>"系统：macos 26"</p>
            <p>"CPU架构：ARM"</p>
            <p>"设备信息：mac mini m4"</p>

        </div>
    }
}
