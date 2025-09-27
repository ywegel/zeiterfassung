use sqlx::Pool;
use sqlx::Sqlite;
use sqlx::sqlite::SqlitePoolOptions;

pub async fn connect_to_database(database_url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}
