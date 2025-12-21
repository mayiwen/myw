pub mod database;
pub mod server;
use std::sync::LazyLock;

use anyhow::Context;
use config::{Config, FileFormat};
use database::DatabaseConfig;
use serde::Deserialize;
use server::ServerConfig;
static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("failed to initialize config"));
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
}
impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config_builder = Config::builder()
            .add_source(
                config::File::with_name("application")
                    .format(FileFormat::Yaml)
                    .required(true),
            )
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            );

        let config = config_builder.build().context("Failed to build config")?;

        // 打印实际加载的配置
        // println!("Loaded config: {:#?}", config);

        config
            .try_deserialize()
            .with_context(|| anyhow::anyhow!("Failed to try_deserialize config"))
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }
    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }
}

pub fn get() -> &'static AppConfig {
    &CONFIG
}
