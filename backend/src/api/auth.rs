use crate::appf::auth::{get_jwt, Principal};
use crate::appf::error::{ApiError, ApiResult};
use crate::appf::middleware::get_auth_layer;
use crate::appf::response::ApiResponse;
use crate::appf::utils::verify_password;
use crate::appf::valid::ValidJson;
use crate::entity::login_user;
use crate::entity::prelude::*;
use axum::extract::{ConnectInfo, State};
use axum::{debug_handler, routing, Extension, Router};
use leptos::config::LeptosOptions;
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use validator::Validate;

pub fn create_router() -> Router<LeptosOptions> {
    Router::new()
        .route("/user-info", routing::get(get_user_info))
        .route_layer(get_auth_layer())
        .route("/login", routing::post(login))
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginParams {
    #[validate(length(min = 3, max = 2000, message = "账号长度为3-2000"))]
    pub name: String,
    #[validate(length(min = 6, max = 2000, message = "密码长度为3-2000"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
// #[serde(rename_all = "camelCase")]
pub struct LoginResult {
    pub access_token: String,
}

#[debug_handler]
#[tracing::instrument(name = "login", skip_all, fields(account = %params.name, ip = %addr.ip()))]
async fn login(
    Extension(db): Extension<DatabaseConnection>,
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

pub async fn ssr_login(params: LoginParams) -> ApiResult<ApiResponse<LoginResult>> {
    tracing::info!("开始处理登录逻辑...");
    let db = crate::get_global_db();
    let user = login_user::Entity::find()
        .filter(login_user::Column::Name.eq(&params.name))
        .one(db)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("账号或密码不正确")))?;
    tracing::info!("密码验证逻辑1");
    if user.login_count > 4 {
        return Err(ApiError::Biz(String::from(
            "登录的次数已用尽，请联系数据库管理员。",
        )));
    };
    if !verify_password(&params.password, &user.password)? {
        // 密码验证不正确，要在登录次数上加1
        // 构建要更新的 ActiveModel（只更新 login_count）
        let mut user_active_model = login_user::ActiveModel::from(user.clone());
        tracing::warn!("当前的user{}", user.clone().login_count);

        // login_count 加 1（注意处理空值，这里默认如果是 null 则按 0 处理）
        user_active_model.login_count = Set(user.clone().login_count + 1);
        // 执行更新操作
        login_user::Entity::update(user_active_model)
            .exec(db)
            .await
            .map_err(|e| {
                // 即使更新失败，也建议记录日志，仍返回原登录错误（避免泄露信息）
                eprintln!("更新登录失败次数失败: {:?}", e);
                ApiError::Biz(String::from("账号或密码不正确"))
            })?;
        return Err(ApiError::Biz(String::from("账号或密码不正确")));
    }
    let principal = Principal {
        id: user.id.to_string(),
        name: user.name.clone(),
    };
    let access_token = get_jwt().encode(principal)?;

    tracing::info!("登录成功, JWT Token: {access_token}");
    let mut user_active_model = login_user::ActiveModel::from(user.clone());
    // login_count 加 1（注意处理空值，这里默认如果是 null 则按 0 处理）
    user_active_model.login_count = Set(0);
    // 执行更新操作
    login_user::Entity::update(user_active_model)
        .exec(db)
        .await
        .map_err(|e| {
            // 即使更新失败，也建议记录日志，仍返回原登录错误（避免泄露信息）
            eprintln!("更新登录失败次数失败: {:?}", e);
            ApiError::Biz(String::from("账号或密码不正确"))
        })?;
    Ok(ApiResponse::ok(
        "登录成功",
        Some(LoginResult { access_token }),
    ))
}
