use std::sync::OnceLock;

use sea_orm::DatabaseConnection;

// crates/server/src/lib.rs
// 导出可被 app crate 引用的模块
pub mod api;
pub mod appf;
pub mod config;
pub mod entity;
// shared/src/db.rs

// 全局静态 DB（仅初始化一次）
static GLOBAL_DB: OnceLock<DatabaseConnection> = OnceLock::new();
// 初始化并设置全局 DB
pub async fn init(db: DatabaseConnection) -> Result<DatabaseConnection, String> {
    // 设置全局 DB（仅第一次有效）
    GLOBAL_DB.set(db.clone()).ok();
    Ok(db)
}
// 获取全局 DB（Server Function 直接调用）
pub fn get_global_db() -> &'static DatabaseConnection {
    GLOBAL_DB.get().expect("全局 DB 未初始化")
}
