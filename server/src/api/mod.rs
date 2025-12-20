use crate::appf::error::{ApiError, ApiResult};
use axum::Router;
use leptos::config::LeptosOptions;
pub mod auth;
pub mod hello;
pub mod link;
pub mod models;
pub mod title;
pub mod user;
use crate::appf::middleware::get_auth_layer;
// pub fn create_router() ->
pub fn create_route() -> Router<LeptosOptions> {
    // Router::new()
    //     .merge(hello::create_route())
    //     .nest(
    //         "/api",
    Router::new()
        .nest("/users", user::create_route())
        .nest("/link", link::index())
        .nest("/title", title::index())
        .route_layer(get_auth_layer())
        .nest("/auth", auth::create_router())
        .nest("/link", link::i())
        .nest("/title", title::i())
    // .fallback(async || -> ApiResult<()> {
    //     tracing::warn!("not found");
    //     Err(ApiError::NotFound)
    // })
    // )
    // .method_not_allowed_fallback(async || -> ApiResult<()> {
    //     tracing::warn!("Method Not Allow");
    //     Err(ApiError::MethodNotAllowed)
    // })
}
