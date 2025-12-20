use crate::config;
use anyhow::Ok;
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement,
};
use std::{cmp::max, time::Duration};

pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let database_config = &config::get().database();
    // println!("{:#?} database config", database_config);
    let mut options = ConnectOptions::new(format!(
        "postgres://{}:{}@{}:{}/{}",
        database_config.user(),
        database_config.password(),
        database_config.host(),
        database_config.port(),
        database_config.database(),
    ));
    tracing::info!("Datebase connected is run ");

    let cpus = num_cpus::get() as u32;
    tracing::info!("Datebase connected is run1 ");
    options
        .min_connections(max(cpus * 4, 10))
        .max_connections(max(cpus * 8, 20))
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(3600 * 24))
        .sqlx_logging(false)
        .set_schema_search_path(database_config.schema());
    tracing::info!("Datebase connected is run2 ");
    let db = Database::connect(options).await?;
    tracing::info!("Datebase connected is run3 ");

    db.ping().await?;
    tracing::info!("Datebase connected successfully");

    log_database_version(&db).await?;
    Ok(db)
}

async fn log_database_version(db: &DatabaseConnection) -> anyhow::Result<()> {
    // 分步处理错误类型转换
    let query_result = db
        .query_one(Statement::from_string(
            DbBackend::Postgres,
            "SELECT version()".to_string(),
        ))
        .await
        .map_err(|e| anyhow::anyhow!("Database query error: {}", e))?; // 转换数据库底层错误类型

    let version_row =
        query_result.ok_or_else(|| anyhow::anyhow!("Failed to get database version"))?; // 处理空结果

    tracing::info!(
        "Database version: {}",
        version_row
            .try_get_by_index::<String>(0)
            .map_err(|e| anyhow::anyhow!("Column access error: {}", e))? // 处理字段解析错误
    );

    Ok(())
}
