use axum::{extract::Request, Router};
use axum::routing::{ get};
pub fn create_route() -> Router {
  let user_app = Router::new()
    .route("/", get(say_hi).patch(say_hi).post(say_hi).delete(say_hi).put(say_hi).options(say_hi))
    ;
  user_app
}
async fn say_hi(request: Request) -> String{
  format!("Welcome to mayiwen.com. This is the backend demonstration page.  anthor: mayiwen. email: i@mayiwen.com.  method is {}", request.method())
}
