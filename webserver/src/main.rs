//! # tavern-goblin

#![warn(missing_docs, clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(
    feature = "rorm-main",
    allow(dead_code, unused_variables, unused_imports)
)]

use std::env;

use clap::Parser;
use rorm::cli as rorm_cli;
use rorm::config::DatabaseConfig;
use rorm::Database;
use rorm::DatabaseConfiguration;
use tracing::instrument;

use crate::cli::Cli;
use crate::cli::Command;
use crate::config::Config;
use crate::global::GlobalEntities;
use crate::global::GLOBAL;

mod cli;
pub mod config;
pub mod global;
pub mod http;
pub mod models;
pub mod utils;

#[instrument(skip_all)]
async fn start(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the database
    let mut conf = DatabaseConfiguration::new(config.database.clone().into());
    conf.disable_logging = Some(true);
    let db = Database::connect(conf).await?;

    // Initialize Globals
    GLOBAL.init(GlobalEntities { db });

    // Start the webserver
    http::server::run(config).await?;

    Ok(())
}

#[rorm::rorm_main]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "INFO");
    }
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let config = Config::try_from_path(&cli.config_path)?;

    match cli.command {
        Command::Start => start(&config).await?,
        Command::Migrate { migrations_dir } => {
            rorm_cli::migrate::run_migrate_custom(
                DatabaseConfig {
                    driver: config.database.into(),
                    last_migration_table_name: None,
                },
                migrations_dir,
                false,
                None,
            )
            .await?
        }
    }

    Ok(())
}
