use crate::{
    myw::{
        self,
        tabset::{Tab, Tabset},
    },
    util::open_url,
};
use leptos::prelude::*;
// 主页面组件（对应原 Index 组件）
#[component]
pub fn I() -> impl IntoView {
    view! {
        <div>
            <myw::Gap />
            <h2>"马一文的个人网站"</h2>

            <p>"建设记录"</p>
            <p>"本站为非商业性质开源网站，仅用于个人学习网站建设技术。"</p>
            <p>"mayiwen.com 基于rust全栈技术，使用服务端渲染（SSR）构建。"</p>
            <myw::Gap h=30 />
            <h2>"鸣谢："</h2>
            <p>"基础：html、css、rust"</p>
            <p>"前端框架：leptos(ssr)"</p>
            <p>"后端框架：axum"</p>
            <p>"orm：seaorm"</p>
            <p>"数据库：postgresSql"</p>
            <p>"服务器：nginx"</p>
            <p>"跨平台客户端：tauri"</p>
            <p>"云服务器、域名：腾迅云"</p>
            <p>"代码平台：https://github.com/mayiwen/myw"</p>
            <p>"js、scss、remixicon、web_sys、serde、tokio"</p>

            <myw::Gap h=30/>
            <h2>"关于我"</h2>
            <p>"前端开发程序员"</p>
            <p>"建议与反馈邮箱：i@mayiwen.com"</p>
            <p>"在github联系我：https://github.com/mayiwen"</p>
            <p>"在gitlink联系我：https://gitlink.org.cn/mayiwen"</p>
            <myw::Gap h=30/>
            <h2>"版本"</h2>
            <p>"当前版本： V1.2.8"</p>
            <p>"2026-07-28 1.2.8 升级leptos至0.9，诺若浏览器添加下载与隐私声明"</p>
            <p>"2026-07-12 1.2.7 添加诺若浏览器0.4.18 beta版本，支持android、ios、windows、macos、linux平台"</p>
            <p>"2026-03-09 1.2.6 添加诺若子网站，对leptos前端框架技术进行二次确认，做了一个艰难的决定，后续不再纠结框架。"</p>
            <p>"2026-01-23 1.2.5 前端框架切到leptos完成，使用ssr重构网站，网站技术栈自此固化"</p>
            <p>"2025-11-01 1.2.4 昨天dioxus发布0.7版本，今日升级"</p>
            <p>"2025-08-10 1.2.1 版本从js切到rust技术栈"</p>
        </div>
    }
}
