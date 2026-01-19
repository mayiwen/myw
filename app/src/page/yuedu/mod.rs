use crate::{
    myw::{
        self,
        button::Button,
        icon,
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::prelude::*;
use leptos_router::components::Outlet;

mod android;
mod ctrl;
mod ios;
mod linux;
mod macos;
mod windows;
/// Renders the home page of your application.
#[component]
pub fn I() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);
    view! {
        <Outlet/>

    }
}
#[component]
pub fn Index() -> impl IntoView {
    let id: RwSignal<u64> = RwSignal::new(0);
    let global_nav =
        use_context::<RwSignal<Option<crate::NavFn>>>().expect("请确保 App 组件已提供全局导航信号");
    view! {
        <div style="text-align: center">
            <div style="text-align: right; margin-right: 8px;">
                <a
                    style="text-decoration: underline; cursor: pointer; line-height: 40px;"
                    on:click= move |_| {
                        global_nav.with(|nav_opt| {
                            if let Some(nav) = nav_opt {
                                nav("/yueduqi/yinsishengming"); // 调用 App 封装的导航逻辑
                            };
                        });
                    }
                    title="https://mayiwen.com/yueduqi/yinsishengming">
                        隐私声明
                </a>
                <myw::Gap w=8/>
                <Button  on_click=move |_| {
                    open_url("https://github.com/mayiwen/yueduqi");
                    }><icon::Github/></Button><myw::Gap w=8/>
                <Button  on_click=move |_| {
                        open_url("https://gitlink.org.cn/mayiwen/yueduqi");
                }><icon::GitLink/></Button>
            </div>
            <div style="text-align: center; max-width: 400px; margin: auto;">
                <myw::Gap h=12/>
                <h1 style="font-weight: bold;">一文小说阅读器</h1>
                <myw::Gap h=12/>
                <Button style="transform: scale(1.2); transform-origin: center"  on_click=move |_| {
                    open_url("https://gitlink.org.cn/mayiwen/yueduqi/releases");
                }>软件与说明书下载</Button>
                <myw::Gap  h=12/>
                <a href="https://github.com/mayiwen/yueduqi/releases" target="_blank">github分流下载</a>
                <myw::Gap h=12/>
                <div style="text-align: left;">
                    <Tabset id=id>
                        <Tab slot id=0 title="ios".to_string()><ios::I/></Tab>
                        <Tab slot id=1 title="android".to_string()><android::I/></Tab>
                        <Tab slot id=2 title="windows".to_string()><windows::I/></Tab>
                        <Tab slot id=3 title="macos".to_string()><macos::I/></Tab>
                        <Tab slot id=4 title="linux".to_string()><linux::I/></Tab>
                        <Tab slot id=5 title="功能".to_string() ><ctrl::I/></Tab>
                    </Tabset>
                </div>
            </div>
            <div style="text-align: center; margin-top: 30px;">
                // p 标签显示阅读器名称
                <p style="color: #666;">"一文小说阅读器"</p>
                <p >
                     <a
                    href="https://www.ccopyright.com.cn/"
                    target="_blank"
                    style="color: #666;"
                    >
                        "软著登字第17310229号"
                    </a>
                </p>
                <p >
                    <a
                        href="https://beian.miit.gov.cn/"
                        target="_blank"
                        style="color: #666;"
                    >
                        "豫ICP备2022018473号-2A"
                    </a>
                </p>
                // a 标签显示备案号并链接到工信部备案平台


            </div>
        </div>
    }
}

#[component]
pub fn YinSiShengMing() -> impl IntoView {
    let global_nav =
        use_context::<RwSignal<Option<crate::NavFn>>>().expect("请确保 App 组件已提供全局导航信号");
    view! {
        <p style="text-align: right; margin-right: 8px;">
            <a
                style="text-decoration: underline; cursor: pointer;"
                on:click= move |_| {
                    global_nav.with(|nav_opt| {
                        if let Some(nav) = nav_opt {
                            nav("/yueduqi"); // 调用 App 封装的导航逻辑
                        };
                    });
                }
                title="https://mayiwen.com/yueduqi">一文小说阅读器主页</a>
        </p>

        <h1 style="text-align: center;">{"一文小说阅读器 隐私声明"}</h1>
        <h3>{"隐私声明最后更新日期：2025-10-25"}</h3>   <myw::Gap/>
        <p>{"本软件为本地软件，不具备联网功能，并不会获取您的个人信息。"}</p><myw::Gap/>
        <p>{"感谢您使用一文小说阅读器。我深知隐私的重要性，并致力于保护您的个人信息。本隐私声明旨在清晰地说明我如何处理您的信息。"}</p><myw::Gap/>
        <h3>{"核心原则：本地处理，绝不外传"}</h3><myw::Gap/>
        <p>{"一文小说阅读器是一款完全运行在您本地设备上的软件。我承诺："}</p><myw::Gap/>
        <p>{"不收集任何个人身份信息。"}</p><myw::Gap/>
        <p>{"我不会主动收集您的姓名、电子邮件地址、电话号码、地理位置等任何可以识别您身份的信息。不联网传输任何数据。"}</p><myw::Gap/>
        <p>{"本软件不具备联网功能，您使用软件产生的所有数据（包括但不限于创建的文件、输入的内容、软件设置等）都仅存储在您的本地设备上，绝不会被发送到我的服务器或任何第三方服务器。 尊重您的数据所有权。"}</p><myw::Gap/>
        <p>{"您使用本软件产生的所有数据，其所有权和控制权完全属于您自己。"}</p><myw::Gap/>
        <h3>{"我如何处理您的数据？ 数据存储："}</h3><myw::Gap/>
        <p>{"您的所有数据（例如项目文件、用户设置、偏好配置等）默认保存在您设备的本地存储空间中（如硬盘、手机内部存储）。您有完全的控制权，可以随时通过软件功能或直接操作文件系统进行修改、备份或删除。"}</p><myw::Gap/>
        <h3>{"数据处理："}</h3><myw::Gap/>
        <p>{"所有的数据处理（如图片渲染、文档编辑、计算分析等）都在您设备的硬件（CPU、GPU、内存）上完成，过程不依赖任何外部网络或远程服务器。"}</p><myw::Gap/>
        <h3>{"第三方服务与链接"}</h3><myw::Gap/>
        <p>{"本软件自身不集成任何需要联网的第三方服务（如广告、分析工具、社交媒体插件等）。本软件在首页展示了获取本软件网站的地址，并不具备点击功能。网站作用是展示软件功能，提供使用说明书。"}</p><myw::Gap/>
        <h3>{"隐私声明的变更"}</h3><myw::Gap/>
        <p>{"如果未来隐私政策发生变更（例如，在未来的软件版本中增加了可选的、需要明确授权的联网功能），我会在更新软件时发布新版隐私声明，并提醒您查阅。"}</p><myw::Gap/>
    }
}
