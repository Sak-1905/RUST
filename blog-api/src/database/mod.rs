// src/database/mod.rs
use sqlx::{PgPool, Pool, Postgres};

pub async fn establish_connection() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await.expect("Failed to create pool")
}