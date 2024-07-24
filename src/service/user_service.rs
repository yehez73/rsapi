use crate::models::user::Users;
use sqlx::PgPool;
use chrono::{NaiveDate, DateTime, Utc};

pub async fn getall_users(pool: &PgPool) -> Result<Vec<Users>, sqlx::Error> {
    
    let rows = sqlx::query_as_unchecked!(
        Users,
        r#"
        SELECT u.user_uuid, uar.user_application_role_uuid, u.user_name, u.user_email, r.role_title, 
            a.application_title, d.division_title, pdm.personal_name, pdm.personal_address, 
            pdm.personal_birthday, pdm.personal_gender, pdm.personal_phone, uar.created_by, 
            uar.created_at, uar.updated_by, uar.updated_at, uar.deleted_by, uar.deleted_at 
        FROM user_ms u 
        INNER JOIN user_application_role_ms uar ON u.user_id = uar.user_id 
        INNER JOIN application_role_ms ar ON uar.application_role_id = ar.application_role_id 
        INNER JOIN application_ms a ON ar.application_id = a.application_id 
        INNER JOIN role_ms r ON ar.role_id = r.role_id 
        INNER JOIN division_ms d ON uar.division_id = d.division_id 
        INNER JOIN personal_data_ms pdm ON u.user_id = pdm.user_id WHERE uar.deleted_at IS NULL
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

// pub async fn create_user(user: User) -> Result<(), sqlx::Error> {
//     let pool: PgPool = establish_connection().await;
//     sqlx::query!(
//         "INSERT INTO users (name) VALUES ($1)",
//         user.name
//     )
//     .execute(&pool)
//     .await?;
//     Ok(())
// }

// pub async fn get_user(id: i32) -> Result<User, sqlx::Error> {
//     let pool: PgPool = establish_connection().await;
//     let row = sqlx::query_as!(
//         User,
//         "SELECT id, name FROM users WHERE id = $1",
//         id
//     )
//     .fetch_one(&pool)
//     .await?;
//     Ok(row)
// }

// pub async fn update_user(id: i32, user: User) -> Result<(), sqlx::Error> {
//     let pool: PgPool = establish_connection().await;
//     sqlx::query!(
//         "UPDATE users SET name = $1 WHERE id = $2",
//         user.name, id
//     )
//     .execute(&pool)
//     .await?;
//     Ok(())
// }

// pub async fn delete_user(id: i32) -> Result<(), sqlx::Error> {
//     let pool: PgPool = establish_connection().await;
//     sqlx::query!(
//         "DELETE FROM users WHERE id = $1",
//         id
//     )
//     .execute(&pool)
//     .await?;
//     Ok(())
// }