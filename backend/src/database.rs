use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

/// Creates a connection pool to the SQLite database and runs migrations.
pub async fn init_pool(db_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    // Run database migrations to create the `posts` table if it doesn't exist.
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS posts (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL, content TEXT NOT NULL)"
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}