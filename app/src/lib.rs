use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock, PoisonError, RwLock};

use crate::models::{Login, SettingTab};
use crate::myw::icon;
use crate::myw::message::{Message, MessageType};
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

pub mod models;
pub mod myw;
pub mod page;
pub mod util;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
      <!DOCTYPE html>
      <html lang="en">
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />
          <AutoReload options=options.clone() />
          <HydrationScripts options />
          <MetaTags />
          <link rel="icon" href="favicon.ico" sizes="any" />
        </head>
        <body>
          <App />
        </body>
      </html>
    }
}

static GLOBAL_TOKEN: OnceLock<RwLock<String>> = OnceLock::new();

// 新增：标记 App 内全局逻辑是否已执行（原子布尔值，线程安全）
static APP_INIT_DONE: AtomicBool = AtomicBool::new(false);

/// 初始化全局 Token（仅第一次调用有效）
pub fn init_global_token(initial_token: String) -> Result<(), &'static str> {
    if initial_token.is_empty() {
        return Err("初始化 Token 不能为空");
    }
    // 初始化时包装成 RwLock（多线程安全）
    GLOBAL_TOKEN
        .set(RwLock::new(initial_token))
        .map_err(|_| "全局 Token 已初始化，不可重复设置")?;
    Ok(())
}

/// 获取全局 Token（多线程安全，同步函数）
pub fn get_global_token() -> String {
    // 1. 获取 OnceLock 中的 RwLock（兼容未初始化，返回空字符串）
    let token_rwlock = match GLOBAL_TOKEN.get() {
        Some(lock) => lock,
        None => return "".to_string(), // 未初始化时返回空，不 panic
    };

    // 2. 读锁（共享锁，多线程可同时读），处理锁污染（PoisonError）
    let token_guard = token_rwlock.read().unwrap_or_else(PoisonError::into_inner);

    token_guard.clone()
}

/// 修改全局 Token（多线程安全，修复 Sync 错误）
pub fn set_global_token(new_token: String) -> Result<(), &'static str> {
    if new_token.is_empty() {
        return Err("修改的 Token 不能为空");
    }

    // 1. 获取 OnceLock 中的 RwLock（未初始化时自动初始化）
    let token_rwlock = match GLOBAL_TOKEN.get() {
        Some(lock) => lock,
        None => {
            // 自动初始化，忽略重复初始化错误
            let _ = GLOBAL_TOKEN.set(RwLock::new(new_token.clone()));
            GLOBAL_TOKEN.get().unwrap() // 初始化后必存在
        }
    };

    // 2. 写锁（排他锁，同一时间仅一个线程可写），处理锁污染
    let mut token_guard = token_rwlock.write().unwrap_or_else(PoisonError::into_inner);

    // 3. 修改内部值（多线程安全）
    *token_guard = new_token;
    Ok(())
}
/// 快捷获取带 Bearer 前缀的 Token
pub fn get_global_token_with_bearer() -> String {
    let pure_token = get_global_token();
    if pure_token.is_empty() {
        "".to_string()
    } else {
        format!("{}", pure_token)
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
    let message = RwSignal::new(vec![
        Message {
            t: MessageType::INFO,
            m: "基于rust全栈技术(axum与leptos ssr)构建".to_string(),
        },
        Message {
            t: MessageType::INFO,
            m: "欢迎访问mayiwen.com".to_string(),
        },
    ]);
    provide_context(message);
    let login = RwSignal::new(Login {
        token: "".to_string(),
    });
    // ========== 核心：全局初始化逻辑（仅执行一次） ==========
    if !APP_INIT_DONE.swap(true, Ordering::SeqCst) {
        // 仅第一次执行 App 组件时进入此分支
        // 1. 初始化全局 Token（捕获错误，避免 unwrap panic）
        if let Err(e) = init_global_token("".to_string()) {}
        // 重复初始化时仅打印日志，不崩溃
    }
    provide_context(login);
    let setting_tab = RwSignal::new(SettingTab { value: 0 });
    provide_context(setting_tab);
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
      <Stylesheet id="leptos" href="/pkg/myw.css" />
      <Title text="马一文个人网站 mayiwen.com | 马一文 mayiwen | 建设纪录" />
      <div style="height: 100%" class="front_box">
        <div style="height: 40px">
          <myw::Gap h=1 />
          <Tabset id=id show_line=false>
            <Tab
              slot
              id=0
              title="马一文".to_string()
              icon=ViewFn::from(move || {
                view! { <icon::Myw></icon::Myw> }
              })
              click=Callback::from(move || { nav("/") })
            >
              ""
            </Tab>
            <Tab
              slot
              id=1
              title="☰ 代码".to_string()
              click=Callback::from(move || { nav("/code") })
            >
              ""
            </Tab>
            <Tab
              slot
              id=2
              title="✤ 阅读".to_string()
              click=Callback::from(move || { nav("/yueduqi") })
            >
              ""
            </Tab>
            <Tab
              slot
              id=3
              title="✯ 浏览".to_string()
              click=Callback::from(move || { nav("/browser") })
            >
              ""
            </Tab>
            <Tab
              slot
              id=4
              title="❃ 设置".to_string()
              click=Callback::from(move || { nav("/setting") })
            >
              ""
            </Tab>
          </Tabset>
          <myw::Gap h=1 />
        </div>
        <div style="height: calc(100% - 40px); overflow: auto;" class="myw-context">
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
                } else if trimmed_path == "browser".to_string() {
                  id.set(3)
                } else if trimmed_path == "setting".to_string() {
                  id.set(4)
                } else {
                  id.set(9999999)
                }
              });
            } <main>
              <Routes fallback=crate::page::not_found::I>
                <Route path=StaticSegment("") view=crate::page::home::I />
                <Route path=StaticSegment("code") view=crate::page::code::I />
                <Route path=StaticSegment("setting") view=crate::page::setting::I />
                <ParentRoute path=StaticSegment("browser") view=crate::page::website::I>
                  <Route path=StaticSegment("") view=crate::page::website::I />
                  <Route
                    path=StaticSegment("yinsishengming")
                    view=crate::page::website::yinsishengming::Yinsishengming
                  />
                </ParentRoute>
                <ParentRoute path=StaticSegment("yueduqi") view=crate::page::yuedu::I>
                  <Route path=StaticSegment("") view=crate::page::yuedu::Index />
                  <Route
                    path=StaticSegment("yinsishengming")
                    view=crate::page::yuedu::YinSiShengMing
                  />
                </ParentRoute>
              </Routes>
            </main>
          </Router>
          <MayiwenBeiAn></MayiwenBeiAn>
          <myw::message::MessageCreate></myw::message::MessageCreate>
        </div>
      </div>
    }
}

#[server(Greet, "/api/ssr/hello")]
pub async fn greet() -> Result<String, ServerFnError> {
    // 服务端逻辑：简单返回字符串
    Ok("Hello from Server Function!".to_string())
}

#[server(GetTitle, "/api/title")]
pub async fn get_title() -> Result<Vec<models::title::Title>, ServerFnError> {
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
        let titles: Vec<models::title::Title> = page
            .items
            .into_iter()
            .map(|model| models::title::Title {
                id: model.id as u64,
                title: model.title,
                index: model.index,
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
#[server(GetLink, "/api/link")]
pub async fn get_link(id: u64) -> Result<Vec<models::link::Link>, ServerFnError> {
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
        let links: Vec<models::link::Link> = page
            .items
            .into_iter()
            .map(|model| models::link::Link {
                id: model.id as u64,
                title: model.title.clone(),
                src: model.src.clone(),
                index: model.index,
                title_id: model.title_id as u64,
            })
            .collect();
        Ok(links)
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::ServerError(
            "此函数仅在服务端可用".to_string(),
        ))
    }
}

#[server(login, "/api/login")]
pub async fn login(name: String, pwd: String) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let params = backend::api::auth::LoginParams {
            name,
            password: pwd,
        };
        use backend::api::auth::LoginResult;
        use backend::appf::response::ApiResponse;

        // 调用 API 获取数据
        let result = backend::api::auth::ssr_login(params).await;

        let async_data: ApiResponse<LoginResult> = match result {
            Ok(data) => data,
            Err(api_error) => {
                // 使用 ServerFnError::ServerError
                return Err(ServerFnError::ServerError(format!(
                    "API调用失败: {}",
                    api_error
                )));
            }
        };
        // 获取到token 信息
        let data: Option<LoginResult> = async_data.data;
        let data: LoginResult = match data {
            Some(page) => page,
            None => return Err(ServerFnError::ServerError("登录入败: ".to_string())),
        };
        Ok(data.access_token)
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::ServerError(
            "此函数仅在服务端可用".to_string(),
        ))
    }
}
#[server(TitleCreate, "/api/ssr/create_title")]
pub async fn create_title(name: String, token: String) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use backend::appf::response::ApiResponse;
        if !backend::appf::middleware::validate_jwt_token(&token) {
            return Err(ServerFnError::ServerError(format!("无权限")));
        }
        // 调用 API 获取数据
        let result = backend::api::title::create_ssr(name).await;

        let async_data: ApiResponse<backend::entity::title::Model> = match result {
            Ok(data) => data,
            Err(api_error) => {
                return Err(ServerFnError::ServerError(format!(
                    "API调用失败: {}",
                    api_error
                )));
            }
        };
        match async_data.data {
            Some(data) => return Ok("添加成功".to_string()),
            None => {
                return Err(ServerFnError::ServerError(format!(
                    "API调用失败: {}",
                    "api_error"
                )));
            }
        };
    }
}

#[server(TitleDeltete, "/api/ssr/title_delete")]
pub async fn title_delete(id: i64, token: String) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use backend::appf::response::ApiResponse;
        if !backend::appf::middleware::validate_jwt_token(&token) {
            return Err(ServerFnError::ServerError(format!("无权限")));
        }
        // 调用 API 获取数据
        let result = backend::api::title::delete_ssr(id).await;

        match result {
            Ok(data) => return Ok("添加成功".to_string()),
            Err(api_error) => {
                return Err(ServerFnError::ServerError(format!(
                    "API调用失败: {}",
                    api_error
                )));
            }
        }
    }
}
#[server(TitleUpdate, "/api/ssr/title_update")]
pub async fn title_update(id: i64, title: String, token: String) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use backend::appf::response::ApiResponse;
        if !backend::appf::middleware::validate_jwt_token(&token) {
            return Err(ServerFnError::ServerError(format!("无权限")));
        }
        // 调用 API 获取数据
        let result = backend::api::title::update_ssr(id, title).await;

        match result {
            Ok(data) => return Ok("添加成功".to_string()),
            Err(api_error) => {
                return Err(ServerFnError::ServerError(format!(
                    "API调用失败: {}",
                    api_error
                )));
            }
        }
    }
}

#[server(LinkCreate, "/api/ssr/create_link")]
pub async fn create_link(
    id: u64,
    title: String,
    src: String,
    token: String,
) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use backend::appf::response::ApiResponse;
        if !backend::appf::middleware::validate_jwt_token(&token) {
            return Err(ServerFnError::ServerError(format!("无权限")));
        }
        let params = backend::api::link::LinkParams {
            title,
            title_id: id as i64,
            src,
        };
        // 调用 API 获取数据
        let result = backend::api::link::create_ssr(params).await;

        let async_data: ApiResponse<backend::entity::link::Model> = match result {
            Ok(data) => data,
            Err(api_error) => {
                return Err(ServerFnError::ServerError(format!(
                    "API调用失败: {}",
                    api_error
                )));
            }
        };
        match async_data.data {
            Some(data) => return Ok("添加成功".to_string()),
            None => {
                return Err(ServerFnError::ServerError(format!(
                    "API调用失败: {}",
                    "api_error"
                )));
            }
        };
    }
}
pub fn get_token() -> String {
    get_global_token_with_bearer()
}

#[server(LinkDelete, "/api/ssr/link_delete")]
pub async fn link_delete(id: i64, token: String) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use backend::appf::response::ApiResponse;
        if !backend::appf::middleware::validate_jwt_token(&token) {
            return Err(ServerFnError::ServerError(format!("无权限")));
        }
        // 调用 API 获取数据
        let result = backend::api::link::delete_ssr(id).await;

        match result {
            Ok(data) => return Ok("添加成功".to_string()),
            Err(api_error) => {
                return Err(ServerFnError::ServerError(format!(
                    "API调用失败: {}",
                    api_error
                )));
            }
        }
    }
}

#[server(LinkUpdate, "/api/ssr/link_update")]
pub async fn link_update(
    id: i64,
    title: String,
    src: String,
    title_id: i64,
    token: String,
) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use backend::appf::response::ApiResponse;
        if !backend::appf::middleware::validate_jwt_token(&token) {
            return Err(ServerFnError::ServerError(format!("无权限")));
        }
        let params = backend::api::link::LinkParams {
            title,
            src,
            title_id,
        };
        // 调用 API 获取数据
        let result = backend::api::link::update_ssr(id, params).await;

        match result {
            Ok(data) => return Ok("添加成功".to_string()),
            Err(api_error) => {
                return Err(ServerFnError::ServerError(format!(
                    "API调用失败: {}",
                    api_error
                )));
            }
        }
    }
}

#[server(LinkSort, "/api/ssr/link_sort")]
pub async fn link_sort(
    vec: Vec<models::link::Link>,
    token: String,
) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        if !backend::appf::middleware::validate_jwt_token(&token) {
            return Err(ServerFnError::ServerError(format!("无权限")));
        }
        let params = vec
            .iter()
            .enumerate()
            .map(|(index, x)| backend::api::link::SortParams {
                id: x.id as i64,
                index: index as i32,
            })
            .collect::<Vec<_>>();

        // 调用 API 获取数据
        let result = backend::api::link::sort_ssr(params).await;
        match result {
            Ok(data) => return Ok("添加成功".to_string()),
            Err(api_error) => {
                return Err(ServerFnError::ServerError(format!(
                    "API调用失败: {}",
                    api_error
                )));
            }
        }
    }
}
#[server(TitleSort, "/api/ssr/title_sort")]
pub async fn title_sort(
    vec: Vec<models::title::Title>,
    token: String,
) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        if !backend::appf::middleware::validate_jwt_token(&token) {
            return Err(ServerFnError::ServerError(format!("无权限")));
        }
        let params = vec
            .iter()
            .enumerate()
            .map(|(index, x)| backend::api::title::SortParams {
                id: x.id as i64,
                index: index as i32,
            })
            .collect::<Vec<_>>();

        // 调用 API 获取数据
        let result = backend::api::title::sort_ssr(params).await;

        match result {
            Ok(data) => return Ok("添加成功".to_string()),
            Err(api_error) => {
                return Err(ServerFnError::ServerError(format!(
                    "API调用失败: {}",
                    api_error
                )));
            }
        }
    }
}
