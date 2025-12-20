use crate::appf::auth::{get_jwt, Principal};
use crate::appf::error::{ApiError, ApiResult};
use crate::appf::middleware::get_auth_layer;
use crate::appf::response::ApiResponse;
use crate::appf::utils::verify_password;
use crate::appf::valid::ValidJson;
use crate::appf::AppState;
use crate::entity::login_user;
use crate::entity::prelude::*;
use axum::extract::{ConnectInfo, State};
use axum::{debug_handler, routing, Extension, Router};
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use validator::Validate;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/user-info", routing::get(get_user_info))
        .route_layer(get_auth_layer())
        .route("/login", routing::post(login))
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginParams {
    #[validate(length(min = 3, max = 2000, message = "账号长度为3-2000"))]
    name: String,
    #[validate(length(min = 6, max = 2000, message = "密码长度为3-2000"))]
    password: String,
}

#[derive(Debug, Serialize)]
// #[serde(rename_all = "camelCase")]
pub struct LoginResult {
    access_token: String,
}

#[debug_handler]
#[tracing::instrument(name = "login", skip_all, fields(account = %params.name, ip = %addr.ip()))]
async fn login(
    State(AppState { db }): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ValidJson(params): ValidJson<LoginParams>,
) -> ApiResult<ApiResponse<LoginResult>> {
    tracing::info!("开始处理登录逻辑...");
    let user = login_user::Entity::find()
        .filter(login_user::Column::Name.eq(&params.name))
        .one(&db)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("账号或密码不正确")))?;

    if !verify_password(&params.password, &user.password)? {
        return Err(ApiError::Biz(String::from("账号或密码不正确")));
    }

    let principal = Principal {
        id: user.id.to_string(),
        name: user.name,
    };
    let access_token = get_jwt().encode(principal)?;

    tracing::info!("登录成功, JWT Token: {access_token}");

    Ok(ApiResponse::ok(
        "登录成功",
        Some(LoginResult { access_token }),
    ))
}

#[debug_handler]
async fn get_user_info(
    Extension(principal): Extension<Principal>,
) -> ApiResult<ApiResponse<Principal>> {
    Ok(ApiResponse::ok("ok", Some(principal)))
}
