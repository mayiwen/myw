// crates/server/src/main.rs
use app::*;
use axum::routing::{any, get};
use axum::Router;
use backend::appf::middleware::get_auth_layer;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};

// 核心修改：从 backend 导入（替换原 server::xxx）
// use backend::api::api_hello_world;
use backend::api::create_route; // 原 server::api::create_route
use backend::appf::{database, run}; // 原 server::appf::run
                                    // use backend::config::get_configuration; // 原 server::config::get_configuration // 补充：原 server::api_hello_world（需确保 backend 导出此函数）

#[tokio::main]
async fn main() {
    // 1. 从 backend 调用配置加载（替换原 server::config）

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);
    let app = Router::new()
        .nest("/api", create_route())
        // .route(
        //     "/ssrf/*rest",
        //     any(|| async { /* 空处理，仅用于挂载中间件 */ }),
        // )
        // .route_layer(get_auth_layer())
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell));

    run(leptos_options, app).await;
}
