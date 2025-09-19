//! # tavern-goblin

#![warn(missing_docs, clippy::unwrap_used, clippy::expect_used)]

use crate::rorm::cli::make_migrations;
use clap::Parser;
use galvyn::core::DatabaseSetup;
use galvyn::core::re_exports::rorm;
use galvyn::rorm::config::DatabaseConfig;
use galvyn::rorm::{Database, DatabaseConfiguration};
use galvyn::{Galvyn, GalvynSetup};
use std::error::Error;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::cli::Cli;
use crate::cli::Command;
use crate::config::{DB, LISTEN_ADDRESS, LISTEN_PORT};

mod cli;
pub mod config;
pub mod http;
pub mod models;
pub mod modules;
pub mod utils;

async fn start() -> Result<(), Box<dyn Error>> {
    Galvyn::builder(GalvynSetup::default())
        .register_module::<Database>(DatabaseSetup::Custom(DatabaseConfiguration::new(
            DB.clone(),
        )))
        .init_modules()
        .await?
        .add_routes(http::initialize_routes())
        .start(SocketAddr::from((
            *LISTEN_ADDRESS.get(),
            *LISTEN_PORT.get(),
        )))
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(errors) = config::load_env() {
        for error in errors {
            eprintln!("{error}");
        }
        return Err("Failed to load configuration".into());
    }

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("DEBUG")))
        .with(tracing_forest::ForestLayer::default())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Command::Start => start().await?,
        #[cfg(debug_assertions)]
        Command::MakeMigrations { migrations_dir } => {
            use std::io::Write;

            const MODELS: &str = "/tmp/.models.json";

            let mut file = std::fs::File::create(MODELS)?;
            rorm::write_models(&mut file)?;
            file.flush()?;

            make_migrations::run_make_migrations(make_migrations::MakeMigrationsOptions {
                models_file: MODELS.to_string(),
                migration_dir: migrations_dir,
                name: None,
                non_interactive: false,
                warnings_disabled: false,
            })?;

            std::fs::remove_file(MODELS)?;
        }
        Command::Migrate { migrations_dir } => {
            rorm::cli::migrate::run_migrate_custom(
                DatabaseConfig {
                    driver: DB.clone(),
                    last_migration_table_name: None,
                },
                migrations_dir,
                false,
                None,
            )
            .await?;
        }
    }

    Ok(())
}
