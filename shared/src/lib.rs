// shared/src/db.rs
// 仅导出 SeaORM 的 DatabaseConnection 类型别名（无任何运行时逻辑）
pub type DbConn = sea_orm::DatabaseConnection;

// 可选：导出通用的 DB 错误类型（供 app crate 处理错误）
pub type DbError = sea_orm::DbErr;
