use crate::appf::common::{Page, PaginationParams};
use crate::appf::error::{ApiError, ApiResult};
use crate::appf::path::Path;
use crate::appf::query::Query;
use crate::appf::response::ApiResponse;
use crate::appf::valid::{ValidJson, ValidQuery};
use crate::appf::AppState;
// use crate::entity::{login_user, prelude::*};
use anyhow::Context;
// use axum::extract::Query;
use crate::entity::link;
use crate::entity::link::ActiveModel;
use axum::Router;
use axum::{debug_handler, extract::State, response::IntoResponse, routing};
use axum_valid::Valid;
use sea_orm::{
    prelude::*, ActiveValue, Condition, IntoActiveModel, Paginator, QueryOrder, QueryTrait,
};
use sea_orm::{DatabaseConnection, TransactionTrait};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
// #[serde(rename_all = "camelCase")]
pub struct LinkQueryParams {
    keyword: Option<String>,
    #[validate(nested)]
    #[serde(flatten)]
    pagination: PaginationParams,
}
#[derive(Debug, Deserialize, Validate, DeriveIntoActiveModel)]
pub struct LinkParams {
    #[validate(length(min = 1, max = 2000, message = "标题长度为1-2000位"))]
    pub title: String,
    // pub index: i32,
    pub src: String,
    pub title_id: i64,
}

pub fn index() -> Router<AppState> {
    Router::new()
        .route("/", routing::post(create))
        .route("/sort", routing::post(sort))
        .route("/{id}", routing::delete(delete))
        .route("/{id}", routing::put(update))
}
pub fn i() -> Router<AppState> {
    Router::new()
        .route("/{id}", routing::get(read_by_title_id))
        .route("/", routing::get(read))
}
#[debug_handler]
async fn read(
    State(AppState { db }): State<AppState>,
    ValidQuery(LinkQueryParams {
        keyword,
        pagination,
    }): ValidQuery<LinkQueryParams>,
) -> ApiResult<ApiResponse<Page<link::Model>>> {
    let paginator = link::Entity::find()
        .apply_if(keyword.as_ref(), |query, keyword| {
            query.filter(
                Condition::any().add(link::Column::Title.contains(keyword)), // .add(link::Column::Account.contains(keyword)),
            )
        })
        .order_by_desc(link::Column::Id)
        .paginate(&db, pagination.size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(pagination.page - 1).await?;
    let page = Page::from_pagination(pagination, total, items);
    Ok(ApiResponse::ok("ok", Some(page)))
}
#[debug_handler]
async fn read_by_title_id(
    State(AppState { db }): State<AppState>,
    ValidQuery(LinkQueryParams {
        keyword,
        pagination,
    }): ValidQuery<LinkQueryParams>,
    Path(id): Path<i64>,
) -> ApiResult<ApiResponse<Page<link::Model>>> {
    let paginator = link::Entity::find()
        .filter(link::Column::TitleId.eq(id)) // 必须按 id 过滤
        // .apply_if(keyword.as_ref(), |query, keyword| {
        //     query.filter(
        //         Condition::any().add(link::Column::TitleId.eq(id)), // .add(link::Column::Account.contains(keyword)),
        //     )
        // })
        .order_by_asc(link::Column::Index)
        .paginate(&db, pagination.size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(pagination.page - 1).await?;
    let page = Page::from_pagination(pagination, total, items);
    Ok(ApiResponse::ok("ok", Some(page)))
}

#[debug_handler]
async fn create(
    State(AppState { db }): State<AppState>,
    ValidJson(params): ValidJson<LinkParams>,
) -> ApiResult<ApiResponse<link::Model>> {
    if params.title.is_empty() {
        return Err(ApiError::Biz(String::from("标题不能为空")));
    }
    let mut active_model = params.into_active_model();
    // active_model.title = ActiveValue::Set(encode_password(&active_model.title.take().unwrap())?);
    let result = active_model.insert(&db).await?;

    Ok(ApiResponse::ok("ok", Some(result)))
}

#[debug_handler]
async fn delete(
    State(AppState { db }): State<AppState>,
    Path(id): Path<i64>,
) -> ApiResult<ApiResponse<()>> {
    let existed_user = link::Entity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("待删除的标题不存在")))?;
    let result = existed_user.delete(&db).await?;
    tracing::info!(
        "Deleted link: {}, affected rows: {}",
        id,
        result.rows_affected
    );

    Ok(ApiResponse::ok("ok", None))
}

#[debug_handler]
async fn update(
    State(AppState { db }): State<AppState>,
    Path(id): Path<i64>,
    ValidJson(params): ValidJson<LinkParams>,
) -> ApiResult<ApiResponse<link::Model>> {
    let existed_user = link::Entity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("修改的标题不存在。")))?;
    let old_title = existed_user.title.clone();
    let new_title = params.title.clone();
    let mut existed_active_model = existed_user.into_active_model();
    let new_src = params.src.clone();
    let new_title_id = params.title_id.clone();
    let mut active_model = params.into_active_model();
    existed_active_model.clone_from(&active_model);
    existed_active_model.id = ActiveValue::Unchanged(id);
    if new_title.is_empty() {
        existed_active_model.title = ActiveValue::Unchanged(old_title);
    } else {
        existed_active_model.title = ActiveValue::Set(new_title);
    }

    existed_active_model.src = ActiveValue::Set(new_src);
    existed_active_model.title_id = ActiveValue::Set(new_title_id);

    let result = existed_active_model.update(&db).await?;

    Ok(ApiResponse::ok("ok", Some(result)))
}
#[derive(Debug, Deserialize, Validate, DeriveIntoActiveModel)]
pub struct SortParams {
    #[validate(range(min = 0, message = "ID 不能小于 0"))]
    pub id: i64,

    #[validate(range(min = 0, message = "索引不能小于 0"))]
    pub index: i32,
}
#[debug_handler]
async fn sort(
    State(AppState { db }): State<AppState>,
    ValidJson(params_list): ValidJson<Vec<SortParams>>,
) -> ApiResult<ApiResponse<Vec<link::Model>>> {
    // 开启事务保证原子性
    let txn = db.begin().await?;

    let mut updated_models = Vec::with_capacity(params_list.len());

    for param in params_list {
        // 1. 检查记录是否存在（自动验证已在ValidJson阶段完成）
        let existed = link::Entity::find_by_id(param.id)
            .one(&txn) // 使用事务连接
            .await?
            .ok_or_else(|| ApiError::Biz(format!("ID {} 对应的记录不存在", param.id)))?;
        // 2. 只更新index字段（保留其他字段不变）
        let mut active_model = existed.into_active_model();
        active_model.index = ActiveValue::Set(param.index); // 关键修改点
                                                            // 3. 执行更新
        let updated = active_model.update(&txn).await?;
        updated_models.push(updated);
    }

    // 提交事务
    txn.commit().await?;

    Ok(ApiResponse::ok("批量更新索引成功", Some(updated_models)))
}
