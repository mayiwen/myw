use crate::appf::common::{Page, PaginationParams};
use crate::appf::error::ApiResult;
use crate::appf::response::ApiResponse;
use crate::appf::valid::ValidQuery;

use crate::entity::{login_user, prelude::*};
use axum::{routing, Extension};
// use axum::extract::Query;
use axum::Router;
use axum::{debug_handler, extract::State};
use leptos::config::LeptosOptions;
use sea_orm::{prelude::*, Condition, QueryOrder, QueryTrait};

use serde::Deserialize;
use validator::Validate;
pub fn create_route() -> Router<LeptosOptions> {
    Router::new()
        .route("/", routing::get(index))
        .route("/find_page", routing::get(find_page).post(find_page))
}
#[derive(Debug, Deserialize, Validate)]
pub struct UserQueryParams {
    keyword: Option<String>,
    #[validate(nested)]
    #[serde(flatten)]
    pagination: PaginationParams,
}
#[debug_handler]
async fn index() -> &'static str {
    "你好"
}

#[tracing::instrument(name = "Query users", skip_all, fields(name = "m"))]
#[debug_handler]
async fn find_page(
    Extension(db): Extension<DatabaseConnection>,
    ValidQuery(UserQueryParams {
        keyword,
        pagination,
    }): ValidQuery<UserQueryParams>,
) -> ApiResult<ApiResponse<Page<login_user::Model>>> {
    tracing::info!("开始处理业务");
    let paginator = LoginUser::find()
        .apply_if(keyword.as_ref(), |query, keyword| {
            query.filter(Condition::any().add(
                login_user::Column::Name.contains(keyword), // .add(login_user::Column::Name.contains(keyword)
            ))
        })
        .order_by_desc(login_user::Column::Id)
        .paginate(&db, pagination.size);
    let users_total = paginator.num_items().await?;
    let items = paginator.fetch_page(pagination.page - 1).await?;
    let page = Page::from_pagination(pagination, users_total, items);
    Ok(ApiResponse::ok("ok", Some(page)))
}
