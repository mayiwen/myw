use crate::myw::icon;
use crate::myw::myw::MayiwenBeiAn;
use crate::myw::tabset::{Tab, Tabset};
#[cfg(feature = "ssr")]
// 明确指定 ServerFnError 的类型参数为 String（解决类型推导错误）
#[cfg(feature = "ssr")]
use backend::DbConn;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::{use_location, use_navigate},
    StaticSegment,
};
#[cfg(feature = "ssr")]
use shared;
pub mod myw;
pub mod page;
pub mod util;
type ServerFnResult<T> = Result<T, ServerFnError<String>>;
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let id: RwSignal<u64> = RwSignal::new(0);
    let (_router_active, set_router_active) = signal("".to_string());
    let nav = |str: &'static str| {
        let navigate = use_navigate();
        navigate(str, Default::default());
    };
    view! {
        <Stylesheet id="leptos" href="/pkg/myw.css"/>
        <Title text="马一文 mayiwen mayiwen.com | 建设纪录"/>
        <Tabset id=id show_line=false>
            <Tab slot id=0 title="马一文".to_string()
                icon={
                    ViewFn::from(move || {
                        view! { <icon::Myw></icon::Myw> }
                    })
                }
                click=Callback::from(move || {
                     nav("/")
                })
                >""</Tab>
            <Tab slot id=1
                title="☰ 代码".to_string()
                click=Callback::from(move || {
                     nav("/code")
                })>""</Tab>
            <Tab slot id=2
                title="✤ 阅读".to_string()
                click=Callback::from(move || {
                     nav("/yueduqi")
                })>""</Tab>
            <Tab slot id=4
                title="❃ 设置".to_string()
                click=Callback::from(move || {
                     nav("/setting")
                })>""</Tab>
        </Tabset>
        // {router_active}
        <Router>
            {
                let location = use_location();
                Effect::new(move |_| {
                    let path = location.pathname.get();
                    let trimmed_path = path.trim_start_matches('/').to_string();
                    set_router_active.set(trimmed_path.clone());
                    if trimmed_path == "code".to_string() {
                        id.set(1);
                    } else if trimmed_path == "".to_string() {
                        id.set(0)
                    } else if trimmed_path == "yueduqi".to_string() {
                        id.set(2)
                    } else if trimmed_path == "setting".to_string() {
                        id.set(4)
                    } else {
                        id.set(9999999)
                    }
                });
            }
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=crate::page::home::I/>
                    <Route path=StaticSegment("/code") view=crate::page::code::I/>
                    <Route path=StaticSegment("/yueduqi") view=crate::page::yuedu::I/>
                    <Route path=StaticSegment("/setting") view=crate::page::setting::I/>
                </Routes>
            </main>
        </Router>

        <MayiwenBeiAn></MayiwenBeiAn>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

// 修复后的 Server Function
// 关键修改：
// 1. 函数标识用 PascalCase（Greet），确保是「纯标识符」（无特殊字符）
// 2. 显式指定返回值的 Error 类型为 ServerFnError
// 3. 函数体用 async + Result
#[server(Greet, "/api/ssr/hello")]
pub async fn greet() -> Result<String, ServerFnError> {
    // 服务端逻辑：简单返回字符串
    Ok("Hello from Server Function!".to_string())
}

#[server(GetTitle, "/api/ssr/title")]
pub async fn get_title() -> ServerFnResult<String> {
    // 仅在服务端（SSR）执行数据库逻辑
    #[cfg(feature = "ssr")]
    {
        // 1. 导入共享的 DB 类型（限定 SSR 特性，避免客户端编译错误）
        use shared::DbConn;

        // 2. 从 Leptos 上下文获取 DB 连接（明确类型+强化错误提示）
        let db = use_context::<DbConn>().ok_or_else(|| {
            ServerFnError::ServerError(
                "❌ 未从上下文获取到数据库连接，请检查：
                    1. 服务端是否在请求处理器内调用 provide_context 注入 DbConn
                    2. DbConn 类型是否与注入的类型完全一致（如是否混用 Arc/裸类型）
                    3. 是否启用了 ssr 特性"
                    .to_string(),
            ) as ServerFnError<String> // 显式指定类型参数
        })?;

        // 3. 调用 backend 的数据库方法（示例：传入 db 执行查询）
        // 替换为你实际的业务函数，注意加 await + 错误转换
        let async_data = backend::api::title::read_ssr(&db).await.map_err(|e| {
            // 显式指定错误类型，解决推导问题
            ServerFnError::ServerError(format!("查询标题失败：{}", e)) as ServerFnError<String>
        })?;

        // 4. 返回实际查询结果（而非固定字符串）
        // return Ok(async_data);
    }

    // 客户端兜底返回（仅为编译通过，不会实际执行）
    Ok("Hello from Server Function!".to_string())
}
