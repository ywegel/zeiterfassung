use sqlx::Pool;
use sqlx::Sqlite;
use sqlx::migrate::MigrateDatabase;
use sqlx::migrate::Migrator;
use sqlx::sqlite::SqlitePoolOptions;

pub async fn connect_to_database(database_url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    if !sqlx::Sqlite::database_exists(database_url).await? {
        sqlx::Sqlite::create_database(database_url).await?;
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    MIGRATOR.run(&pool).await?;

    Ok(pool)
}

pub static MIGRATOR: Migrator = sqlx::migrate!();
