use app::*;
use axum::routing::get;
use axum::Router;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};

use crate::api::create_route;

mod api;
mod appf;
mod config;
mod entity;
#[tokio::main]
async fn main() {
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    // let db = appf::database::init().await.expect("数据库连接错误！！！");

    let app = Router::new()
        .route("/hello", get(api_hello_world))
        .nest("/api", create_route())
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell));
    appf::run(leptos_options, app).await;
    // .with_state(core_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    // log!("listening on http://{}", &addr);
    // let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    // axum::serve(listener, app.into_make_service())
    //     .await
    //     .unwrap();
}
// 更复杂的处理器示例
async fn api_hello_world() -> axum::response::Json<serde_json::Value> {
    axum::response::Json(serde_json::json!({
        "message": "Hello from API!",
        "status": "success"
    }))
}
