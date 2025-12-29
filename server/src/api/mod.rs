use crate::appf::error::{ApiError, ApiResult};
use app;
use leptos::config::LeptosOptions;
use leptos::*;
use leptos_axum::*;
pub mod auth;
pub mod hello;
pub mod link;
pub mod models;
pub mod title;
pub mod user;
use crate::appf::middleware::get_auth_layer;
use axum::{
    routing::{get, post},
    Router,
};

// pub fn create_router() ->
pub fn create_route() -> Router<LeptosOptions> {
    Router::new()
        .nest("/users", user::create_route())
        .nest("/link", link::index())
        .nest("/title", title::index())
        // .route("/ssr/*fn_name", get(handle_server_fns))
        .route_layer(get_auth_layer())
        .nest("/auth", auth::create_router())
        .nest("/link", link::i())
        .nest("/title", title::i())
}
