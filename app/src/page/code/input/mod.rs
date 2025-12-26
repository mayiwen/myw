use crate::{
    myw::{
        self,
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::prelude::*;
/// Renders the home page of your application.
#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);
    view! {
        <div style="text-align: center; ">
            <myw::Gap/>
            <h1>
                马一文的输入法
            </h1>
            <myw::Gap h=30/>
            <myw::button::I style="transform: scale(1.4); transform-origin: center"  on:click=move |_| {
                open_url("https://gitlink.org.cn/mayiwen/nuoruo/releases");
            }>获取开源软件</myw::button::I>
            <myw::Gap h=30/>
            <h3>"windows、android提供安装包"</h3>
            <p>"windows 基于多多输入法打包"</p>
            <p>"android 基于同文输入法打包"</p>
            <myw::Gap/>
            <h3>"其他平台，提供通用方案。"</h3>
            <p>"下载方案后，可部署到rime输入法平台以实现多端兼容"</p>
            <p>"下面是rime输入法在各平台的输入法名称"</p>
            <p>"windows 小狼毫输入法"</p>
            <p>"mac 鼠鬚管输入法"</p>
            <p>"linux 中州韻输入法"</p>
            <p>"andorid 同文输入法"</p>
            <p>"ios 仓输入法"</p>
            <myw::Gap/>
            "ios 安卓提供自用输入法主题"
        </div>
        <div style="text-align: center; max-width: 400px; margin: auto;">
            <myw::Gap h=12/>
            <div style="text-align: left;">
                <Tabset id=id>
                    <Tab slot id=0 title="ios".to_string()>
                        <myw::Gap h=12/>
                        <h3>ios</h3>
                        <myw::Gap h=12/>
                        <img src="img/input-method/ios-input.jpg" class="myw-img"/>
                    </Tab>
                    <Tab slot id=1 title="android".to_string()>
                        <myw::Gap h=12/>
                        <h3>android</h3>
                        <myw::Gap h=12/><myw::Gap h=12/>
                        <img src="img/input-method/android-rime.jpg" class="myw-img"/>
                    </Tab>
                    <Tab slot id=2 title="windows".to_string()>
                        <myw::Gap h=12/>
                        <h3>windows</h3>
                        <myw::Gap h=12/>
                        <img src="img/input-method/windows-duoduo.png" class="myw-img"/>
                    </Tab>
                    <Tab slot id=3 title="macos".to_string()>
                        <myw::Gap h=12/>
                        <h3>macos</h3>
                        <myw::Gap h=12/>
                        <img src="img/input-method/macos.png" class="myw-img"/>
                    </Tab>
                    <Tab slot id=4 title="linux".to_string()>
                        <myw::Gap h=12/>
                        <h3>linux</h3>
                        <myw::Gap h=12/>
                        <img src="img/input-method/linux-rime.png" class="myw-img"/>
                    </Tab>
                </Tabset>
            </div>
        </div>
    }
}
