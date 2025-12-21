use crate::myw;
use leptos::prelude::*;

#[component]
pub fn I() -> impl IntoView {
    view! {
        <div style=" max-width: 400px; margin: auto;">
            <myw::Gap/>
            <h3>"linux 主界面"</h3>
            <myw::Gap/>
            <img style="width: 100%; max-width: 400px;" src="img/nuoruo/yueduqi/yueduqi_deb.png" alt="linux image" />
            <h4>"截图来源"</h4>
            <p>"系统：Ubuntu 25.04 64位"</p>
            <p>"CPU架构：AMD64"</p>
            <p>"设备信息：组装机 Intel(R) Xeon(R) CPU E5-1650 v2"</p>
        </div>
    }
}
