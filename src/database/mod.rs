use sqlx::PgPool;
use actix_web::web;
use std::env;
use dotenv::dotenv;

pub async fn init_pool() -> web::Data<PgPool> {
    let pool = establish_connection().await;
    web::Data::new(pool)
}

pub async fn establish_connection() -> PgPool {
    dotenv().ok(); // Memuat variabel dari file .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await.unwrap()
}