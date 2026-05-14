use std::{env, path::PathBuf};

use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use tracing::*;
use tracing_subscriber::EnvFilter;
use weather_app_server::{Config, Database};

#[tokio::main]

async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true)
        .with_line_number(true)
        .without_time()
        .init();

    let config_path = env::var("CONFIG").expect("missing environment variable CONFIG");
    let config = Config::load(&PathBuf::from(config_path)).expect("failed to load config");
    Database::init(&config.sqlite_db_path).expect("failed to setup db");

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");
    debug!("Running migrations");
    let migrations_count = Database::conn()
        .run_pending_migrations(MIGRATIONS)
        .unwrap()
        .len();
    debug!("Successfully ran {migrations_count} migrations");

    // init done
}
