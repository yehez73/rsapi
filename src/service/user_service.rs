use crate::models::user::User;
use crate::database::establish_connection;
use sqlx::PgPool;

pub async fn getall_user() -> Result<Vec<User>, sqlx::Error> {
    let pool: PgPool = establish_connection().await;
    let rows = sqlx::query_as!(
        User,
        "SELECT id, name FROM users"
    )
    .fetch_all(&pool)
    .await?;
    Ok(rows)
}

pub async fn create_user(user: User) -> Result<(), sqlx::Error> {
    let pool: PgPool = establish_connection().await;
    sqlx::query!(
        "INSERT INTO users (name) VALUES ($1)",
        user.name
    )
    .execute(&pool)
    .await?;
    Ok(())
}

pub async fn get_user(id: i32) -> Result<User, sqlx::Error> {
    let pool: PgPool = establish_connection().await;
    let row = sqlx::query_as!(
        User,
        "SELECT id, name FROM users WHERE id = $1",
        id
    )
    .fetch_one(&pool)
    .await?;
    Ok(row)
}

pub async fn update_user(id: i32, user: User) -> Result<(), sqlx::Error> {
    let pool: PgPool = establish_connection().await;
    sqlx::query!(
        "UPDATE users SET name = $1 WHERE id = $2",
        user.name, id
    )
    .execute(&pool)
    .await?;
    Ok(())
}

pub async fn delete_user(id: i32) -> Result<(), sqlx::Error> {
    let pool: PgPool = establish_connection().await;
    sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        id
    )
    .execute(&pool)
    .await?;
    Ok(())
}