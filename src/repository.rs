use sqlx::{Error, SqlitePool};
use crate::db::establish_connection;
use crate::models::Employee;

pub struct Repository {
    pool: SqlitePool,
}

impl Repository {
    pub async fn new() -> Result<Self, Error> {
        let pool = establish_connection().await?;
        Ok(Repository { pool })
    }

    pub async fn get_employees(&self) -> Result<Vec<Employee>, Error> {
        sqlx::query_as::<_, Employee>("SELECT * FROM Employee")
            .fetch_all(&self.pool)
            .await
    }
}