use actix_web::{Error, HttpResponse};
use sqlx::PgPool;
use bcrypt::{hash, DEFAULT_COST};
use base64::{engine::general_purpose, Engine as _};
use uuid::Uuid;
use crate::models::user::{Register, Users};

pub async fn getall_users(pool: &PgPool) -> Result<Vec<Users>, sqlx::Error> {
    let query = r#"
        SELECT 
            u.user_uuid, 
            uar.user_application_role_uuid, 
            u.user_name, 
            u.user_email, 
            r.role_title, 
            a.application_title, 
            d.division_title, 
            pdm.personal_name, 
            pdm.personal_address, 
            pdm.personal_birthday, 
            pdm.personal_gender, 
            pdm.personal_phone, 
            uar.created_by, 
            uar.created_at,
            uar.updated_by,
            uar.updated_at,
            uar.deleted_by,
            uar.deleted_at
        FROM user_ms u 
        INNER JOIN user_application_role_ms uar ON u.user_id = uar.user_id 
        INNER JOIN application_role_ms ar ON uar.application_role_id = ar.application_role_id 
        INNER JOIN application_ms a ON ar.application_id = a.application_id 
        INNER JOIN role_ms r ON ar.role_id = r.role_id 
        INNER JOIN division_ms d ON uar.division_id = d.division_id 
        INNER JOIN personal_data_ms pdm ON u.user_id = pdm.user_id 
        WHERE uar.deleted_at IS NULL
    "#;

    let users = sqlx::query_as::<_, Users>(query)
        .fetch_all(pool)
        .await?;

    Ok(users)
}   

pub async fn add_user(pool: &PgPool,user: Register) -> Result<HttpResponse, Error> {

    if user.user_password.len() < 8 {
        return Err(actix_web::error::ErrorBadRequest("Password must be at least 8 characters"));
    }

    let user_uuid = "5b9909a5-216e-4e20-95a3-8b9b0114b401";

    let hashed_password = hash(user.user_password, DEFAULT_COST).unwrap();

    let hashed_password_str = general_purpose::STANDARD.encode(hashed_password.as_bytes());

    let username = get_username_by_id(pool, user_uuid).await.unwrap();

    // buat current timestamp untuk created_at dengan library naivedate library dengan output dd/mm/yy dan jam:menit:detik
    let current_timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let user_uuid = Uuid::new_v4();

    println!("New password created {} at {} with UUID {} created by {}", hashed_password_str, current_timestamp, user_uuid, username);

    Ok(HttpResponse::Ok().finish())
}

async fn get_username_by_id(db_pool: &PgPool, user_uuid: &str) -> Result<String, sqlx::Error> {
    let username = sqlx::query!(
        "SELECT user_name FROM user_ms WHERE user_uuid = $1",
        user_uuid
    )
    .fetch_one(db_pool)
    .await?
    .user_name;
    Ok(username)
}