use axum::routing::get;
use axum::{extract::Request, Router};

use crate::appf::AppState;
// hello.rs
pub fn create_route() -> Router<AppState> {
    Router::new().route(
        "/",
        get(say_hi)
            .patch(say_hi)
            .post(say_hi)
            .delete(say_hi)
            .put(say_hi)
            .options(say_hi),
    )
}

async fn say_hi(request: Request) -> String {
    format!("Welcome to mayiwen.com. This is the backend demonstration page.  anthor: mayiwen. email: i@mayiwen.com.  method is {}", request.method())
}
