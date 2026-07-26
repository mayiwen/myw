use crate::myw;
use leptos::prelude::*;
use myw::button::Button;

#[component]
pub fn YinSiShengMing() -> impl IntoView {
    let global_nav =
        use_context::<RwSignal<Option<crate::NavFn>>>().expect("请确保 App 组件已提供全局导航信号");
    view! {
        <div style="padding: 20px; margin: 0 auto;">
            <p style="text-align: right; margin-bottom: 20px;">
                  <Button  on_click=move |_| {
                       global_nav.with(|nav_opt| {
                        if let Some(nav) = nav_opt {
                            nav("/browser"); // 调用 App 封装的导航逻辑
                        };
                    });
                    }>诺若浏览器主页</Button><myw::Gap w=8/>

            </p>

            <h1 style="text-align: center; font-size: 24px; font-weight: bold; margin-bottom: 20px;">{"诺若浏览器 隐私声明"}</h1>
            <h3 style="color: #666; margin-bottom: 16px;">{"隐私声明最后更新日期：22026-07-25"}</h3>
            <myw::Gap h=16 />

            <p style="line-height: 1.8; color: #333;">{"欢迎您使用诺若浏览器，本软件不会收集您的任何信息，所有的数据都存储在您的本地设备上。"}</p>
            <myw::Gap h=16 />

            <h3 style="font-size: 18px; font-weight: bold; margin-top: 24px; margin-bottom: 12px;">{"您信息的处理方式"}</h3>
            <myw::Gap h=12 />
            <p style="line-height: 1.8; color: #333;">{"首页导航信息与主页网址，基于web localStorage技术存储,存于您设备的本地localStorage中。"}</p>

            <myw::Gap h=12 />
            <p style="line-height: 1.8; color: #333;">{"访问历史与下载历史记录，使用sqllite数据库存储,存于您设备的本地数据库中。该数据库位于应用安装目录的history.db等文件中。"}</p>
            <myw::Gap h=12 />

            <p style="line-height: 1.8; color: #333;">{"下载的文件，会存储在您设备的本地本应用私用目录中。不会访问到您的其他文件。"}</p>
            <myw::Gap h=12 />

            <h3 style="font-size: 18px; font-weight: bold; margin-top: 24px; margin-bottom: 12px;">{"个人承诺"}</h3>
            <myw::Gap h=12 />
            <p style="line-height: 1.8; color: #333;">{"本浏览器在开发过程中，不会设计任何发送您个人数据到我的服务器的行为。"}</p>
            <myw::Gap h=12 />

            <h3 style="font-size: 18px; font-weight: bold; margin-top: 24px; margin-bottom: 12px;">{"链接跳转说明"}</h3>
            <myw::Gap h=12 />
            <p style="line-height: 1.8; color: #333;">{"本软件会提供链接跳转功能，部分可能会跳转至mayiwen.com，此为获取软件更新及使用帮助。不会收集您的任何个人信息。"}</p>
            <p style="line-height: 1.8; color: #333;">{"搜索功能将调用百度搜索，输入网址会跳转第三方网站。这些网站可能会收集您的信息，请查阅各个网站的隐私政策以了解他们的数据处理方式。"}</p>
            <myw::Gap h=12 />

            <h3 style="font-size: 18px; font-weight: bold; margin-top: 24px; margin-bottom: 12px;">{"隐私声明的变更"}</h3>
            <myw::Gap h=12 />
            <p style="line-height: 1.8; color: #333;">{"如果未来隐私政策发生变更，我会在更新软件时发布新版隐私声明，并提醒您查阅。"}</p>
            <myw::Gap h=12 />
        </div>
    }
}
