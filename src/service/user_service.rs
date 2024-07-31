use actix_web::{web::{self}, Error, HttpResponse};
use sqlx::{PgPool};
use bcrypt::{hash, verify, DEFAULT_COST};
use base64::{engine::general_purpose, Engine as _};
use crate::models::user::{self, Register, Users};

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

    let hashed_password = hash(user.user_password, DEFAULT_COST).unwrap();

    let hashed_password_str = general_purpose::STANDARD.encode(hashed_password.as_bytes());

    // let username = GetUsernameByID(pool, userUUID).await?;

    Ok(HttpResponse::Ok().finish())
}

// pub async fn GetUsernameByID(pool: &PgPool, userUUID: String) -> Result<String, Error> {
//     let row: (String,) = sqlx::query_as("SELECT user_name FROM user_ms WHERE user_uuid = $1")
//         .bind(userUUID)
//         .fetch_one(pool)
//         .await
//         .unwrap();
    
//     Ok(row.0)
// }