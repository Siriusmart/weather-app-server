use std::{env, path::PathBuf};

use axum::{Router, routing::get};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use tokio::net::TcpListener;
use tracing::*;
use tracing_subscriber::EnvFilter;
use weather_app_server::{Config, Database, api};

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
    let app = Router::new()
        .route("/", get(|| async { "See API docs on https://github.com/Interaction-Design-Team/weather-app-server" }))
        .nest("/api", api::scope());

    let listener = TcpListener::bind(("0.0.0.0", config.port)).await.unwrap();

    info!("Listening on 0.0.0.0:{}", config.port);
    axum::serve(listener, app).await.unwrap();
}
