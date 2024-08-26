use crate::models::auth::{Login, UserDetails};
use sqlx::PgPool;
use bcrypt::{verify};
use std::{error::Error, str};
use base64::{self, engine::general_purpose, Engine};

pub async fn login(pool: &PgPool, login: &Login) -> Result<UserDetails, Box<dyn Error>> {
    // Check if the user exists
    let query = sqlx::query!(
        "SELECT user_uuid, user_password FROM user_ms WHERE user_email = $1",
        login.user_email
    )
    .fetch_one(pool)
    .await;

    let (user_uuid, db_password_base64) = match query {
        Ok(row) => (row.user_uuid, row.user_password),
        Err(_) => return Err("Email doesn`t exist".into()),
    };

    let input_password = login.user_password.to_string();
    let db_password_bytes = general_purpose::STANDARD.decode(db_password_base64)?;
    let db_password = str::from_utf8(&db_password_bytes)?;

    if verify(input_password, db_password)? == false {
        return Err("Password is incorrect".into());
    }

    // Authentication succeeded, fetch additional details
    let query_division = sqlx::query!(
        "SELECT d.division_title, d.division_code FROM division_ms d
        JOIN user_application_role_ms uar ON d.division_id = uar.division_id
        JOIN user_ms u ON uar.user_id = u.user_id
        WHERE u.user_uuid = $1",
        user_uuid
    )
    .fetch_one(pool)
    .await?;

    let division_title = query_division.division_title;
    let division_code = query_division.division_code;

    let query_role = sqlx::query!(
        "SELECT r.role_code FROM role_ms r
        JOIN application_role_ms ar ON r.role_id = ar.role_id
        JOIN user_application_role_ms uar ON ar.application_role_id = uar.application_role_id
        JOIN user_ms u ON u.user_id = uar.user_id
        WHERE u.user_uuid = $1",
        user_uuid
    )
    .fetch_one(pool)
    .await?;

    let role_code = query_role.role_code;

    let query_username = sqlx::query!(
        "SELECT user_name FROM user_ms WHERE user_uuid = $1",
        user_uuid
    )
    .fetch_one(pool)
    .await?;

    let username = query_username.user_name;

    Ok(UserDetails {
        user_uuid,
        role_code,
        division_title,
        division_code,
        username,
    })
}