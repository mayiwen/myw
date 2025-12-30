// crates/server/src/main.rs
use app::*;
use axum::routing::get;
use axum::Router;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};

// 核心修改：从 backend 导入（替换原 server::xxx）
// use backend::api::api_hello_world;
use backend::api::create_route; // 原 server::api::create_route
use backend::appf::run; // 原 server::appf::run
                        // use backend::config::get_configuration; // 原 server::config::get_configuration // 补充：原 server::api_hello_world（需确保 backend 导出此函数）

#[tokio::main]
async fn main() {
    // 1. 从 backend 调用配置加载（替换原 server::config）
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    // 2. 构建路由：所有原 server/lib.rs 的函数替换为 backend 导入
    let app = Router::new()
        // .route("/hello", get(api_hello_world)) // 调用 backend 的 api_hello_world
        .nest("/api", create_route()) // 调用 backend 的 create_route
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell));

    // 3. 调用 backend 的 run 方法（替换原 server::appf::run）
    run(leptos_options, app).await;
}
