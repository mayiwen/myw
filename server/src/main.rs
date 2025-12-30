use app::*;
use axum::routing::get;
use axum::Router;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};

// 从 lib.rs 引用模块（而非本地 mod）
use server::api::create_route;
use server::appf::run;
use server::config::get_configuration;

#[tokio::main]
async fn main() {
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/hello", get(server::api_hello_world)) // 引用 lib.rs 导出的函数
        .nest("/api", create_route())
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell));

    run(leptos_options, app).await;
}
