use crate::myw::icon;
use crate::myw::myw::MayiwenBeiAn;
use crate::myw::tabset::{Tab, Tabset};
use serde::{Deserialize, Serialize};
// 明确指定 ServerFnError 的类型参数为 String（解决类型推导错误）
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    hooks::{use_location, use_navigate},
    path, StaticSegment,
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

// 1. 定义导航函数的类型（入参是 &'static str，无返回值）
pub type GlobalNavFn = Callback<&'static str, ()>;
// 2. 定义上下文的唯一 Key（避免和其他上下文冲突）
// 定义导航函数的类型别名（入参是 &'static str）
pub type NavFn = Box<dyn Fn(&'static str) + Send + Sync>;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let id: RwSignal<u64> = RwSignal::new(0);
    let (_router_active, set_router_active) = signal("".to_string());
    let global_nav: RwSignal<Option<NavFn>> = RwSignal::new(None);
    let nav = |str: &'static str| {
        let navigate = use_navigate();
        navigate(str, Default::default());
    };
    // 4. 【关键】把信号提供给子组件（用 provide_context，无需 ContextKey）
    provide_context(global_nav);
    // 3. 将 nav 逻辑存入全局信号（Box 封装成 trait 对象）
    global_nav.set(Some(Box::new(nav)));
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
                    <Route path=StaticSegment("code") view=crate::page::code::I/>
                    // <Route path!="yueduqi" view=crate::page::yuedu::I/>
                    <Route path=StaticSegment("setting") view=crate::page::setting::I/>
                    <ParentRoute path=StaticSegment("yueduqi") view=crate::page::yuedu::I>
                        // 默认子路由（访问 /yueduqi 时渲染 Index）
                        <Route path=StaticSegment("") view=crate::page::yuedu::Index/>
                        <Route path=StaticSegment("yinsishengming") view=crate::page::yuedu::YinSiShengMing/>
                    </ParentRoute >
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    pub id: u64,
    pub title: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub id: u64,
    pub title: String,
    pub src: String,
}
#[server(GetTitle, "/api/ssr/title")]
pub async fn get_title() -> Result<Vec<Title>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use backend::appf::{common::Page, response::ApiResponse};

        // 调用 API 获取数据
        let result = backend::api::title::read_ssr().await;

        let async_data: ApiResponse<Page<backend::entity::title::Model>> = match result {
            Ok(data) => data,
            Err(api_error) => {
                // 使用 ServerFnError::ServerError
                return Err(ServerFnError::ServerError(format!(
                    "API调用失败: {}",
                    api_error
                )));
            }
        };

        // 检查数据是否存在
        let page = async_data.data;
        let page = match page {
            Some(page) => page,
            None => return Err(ServerFnError::ServerError("API调用失败: ".to_string())),
        };
        // 转换模型
        let titles: Vec<Title> = page
            .items
            .into_iter()
            .map(|model| Title {
                id: model.id as u64,
                title: model.title,
            })
            .collect();

        Ok(titles)
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::ServerError(
            "此函数仅在服务端可用".to_string(),
        ))
    }
}
#[server(GetLink, "/api/ssr/link")]
pub async fn get_link(id: u64) -> Result<Vec<Link>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use backend::appf::{common::Page, response::ApiResponse};

        // 调用 API 获取数据
        let result = backend::api::link::read_ssr(id).await;

        let async_data: ApiResponse<Page<backend::entity::link::Model>> = match result {
            Ok(data) => data,
            Err(api_error) => {
                // 使用 ServerFnError::ServerError
                return Err(ServerFnError::ServerError(format!(
                    "API调用失败: {}",
                    api_error
                )));
            }
        };

        // 检查数据是否存在
        let page = async_data.data;
        let page = match page {
            Some(page) => page,
            None => return Err(ServerFnError::ServerError("API调用失败: ".to_string())),
        };
        // 转换模型
        let links: Vec<Link> = page
            .items
            .into_iter()
            .map(|model| Link {
                id: model.id as u64,
                title: model.title.clone(),
                src: model.src.clone(),
            })
            .collect();
        // pub id: i64,
        // pub title: String,
        // pub src: String,
        // pub title_id: i64,
        // pub index: i32,

        Ok(links)
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::ServerError(
            "此函数仅在服务端可用".to_string(),
        ))
    }
}
