pub mod delete_data;
pub mod get_data;
pub mod update_data;

use sqlx::{Executor, Pool, Sqlite, sqlite::SqlitePoolOptions, query};

pub async fn create_pool() -> Pool<Sqlite>{
    let data_base_url = "sqlite://database/database.db";
    let pool = SqlitePoolOptions::new().connect(data_base_url).await.expect("Connection Error");

    query(include_str!("../../database/migrations/0001_create_table.sql"))
        .execute(&pool)
        .await
        .expect("Query Error");

    return pool;
}