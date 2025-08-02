use sqlx::postgres::{PgPool, PgPoolOptions};

/// Creates a connection pool to the PostgreSQL database and runs migrations.
pub async fn init_pool(db_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    // Run database migrations to create the `posts` table if it doesn't exist.
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS posts (id SERIAL PRIMARY KEY, title TEXT NOT NULL, content TEXT NOT NULL)"
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}