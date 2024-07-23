use sqlx::PgPool;

pub async fn establish_connection() -> PgPool {
    let database_url = "postgres://postgres:123@localhost/ainodocs";
    PgPool::connect(&database_url).await.unwrap()
}