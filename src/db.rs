use sqlx::{Error, SqlitePool};

pub async fn establish_connection() -> Result<SqlitePool, Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePool::connect(&database_url).await
}