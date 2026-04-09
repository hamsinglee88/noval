use std::{path::Path, str::FromStr};

use sqlx::{
    SqlitePool,
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn create_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    ensure_parent_directory(database_url)?;

    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true);

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::migrate::MigrateError> {
    MIGRATOR.run(pool).await
}

fn ensure_parent_directory(database_url: &str) -> Result<(), sqlx::Error> {
    if let Some(path) = database_url.strip_prefix("sqlite://") {
        if path == ":memory:" {
            return Ok(());
        }

        if let Some(parent) = Path::new(path).parent() {
            std::fs::create_dir_all(parent).map_err(sqlx::Error::Io)?;
        }
    }

    Ok(())
}
