use crate::myw::icon;
use crate::myw::myw::MayiwenBeiAn;
use crate::myw::tabset::{Tab, Tabset};
// 明确指定 ServerFnError 的类型参数为 String（解决类型推导错误）
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::{use_location, use_navigate},
    StaticSegment,
};

pub mod myw;
pub mod page;
pub mod util;

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

type ServerFnResult<T> = Result<T, ServerFnError<String>>;
// 第三步：无 Serde、无报错的完整实现
#[server(GetTitle, "/api/ssr/title")]
pub async fn get_title() -> ServerFnResult<String> {
    // // 仅在服务端执行数据库逻辑
    #[cfg(feature = "ssr")]
    {
        // 1. 调用数据库方法并处理错误（无 Serde）
        let async_data = backend::api::title::read_ssr()
            .await
            // 错误转换：任意错误 → 字符串形式的 ServerFnError
            .map_err(|e| ServerFnError::ServerError(format!("查询失败：{}", e)))?;

        // 2. 提取复杂类型中的字符串（核心：避开 Serde，直接取可用字段）
        // 示例1：如果返回 ApiResponse，提取 msg 字段
        let result_str = "async_data.msg".to_string().clone();
        // 示例2：如果返回 Page<Model>，拼接列表文本（根据你的实际结构调整）
        // let result_str = format!("共{}条数据", async_data.total);
        // 示例3：如果仅需固定文本，直接返回
        // let result_str = "查询成功".to_string();

        // 3. 返回纯字符串结果（无 Serde 依赖）
        return Ok(result_str);
    }

    // 客户端兜底返回（纯字符串，无 Serde）
    Ok("Hello from Server Function!".to_string())
}
