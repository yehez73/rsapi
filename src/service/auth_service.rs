use crate::models::auth::{ChangePassword, Login, UserDetails};
use sqlx::{query, PgPool};
use bcrypt::{hash, verify, DEFAULT_COST};
use core::str;
use std::{error::Error};
use base64::{self, engine::general_purpose, Engine};

pub async fn login(pool: &PgPool, user_login: &Login) -> Result<UserDetails, Box<dyn Error>> {
    let mut is_authenticated = false;
    let mut user_id: i64 = 0;
    let mut user_uuid = String::new();
    let mut role_code = String::new();
    let mut division_title = String::new();
    let mut division_code = String::new();
    let mut user_name = String::new();

    if let Some(row) = query!(
        "SELECT user_uuid, user_password, user_id 
         FROM user_ms 
         WHERE user_email = $1",
        user_login.user_email
    )
    .fetch_optional(pool)
    .await?
    {
        user_uuid = row.user_uuid;
        user_id = row.user_id;

        println!("DB Password: {}", row.user_password);

        let input_password = user_login.user_password.to_string();
        let db_password_bytes = general_purpose::STANDARD.decode(row.user_password)?;
        let db_password = str::from_utf8(&db_password_bytes)?;

        if verify(input_password, db_password)? == false {
            return Err("Password is incorrect".into());
        } else {
            is_authenticated = true;
        }
    }

    if is_authenticated {
        if let Some(row) = query!(
            "SELECT d.division_title, d.division_code 
             FROM division_ms d 
             JOIN user_application_role_ms uar ON d.division_id = uar.division_id 
             JOIN user_ms u ON uar.user_id = u.user_id 
             WHERE u.user_uuid = $1",
            user_uuid
        )
        .fetch_optional(pool)
        .await?
        {
            division_title = row.division_title;
            division_code = row.division_code;
        }

        if let Some(row) = query!(
            "SELECT r.role_code 
             FROM role_ms r 
             JOIN application_role_ms ar ON r.role_id = ar.role_id 
             JOIN user_application_role_ms uar ON ar.application_role_id = uar.application_role_id 
             JOIN user_ms u ON u.user_id = uar.user_id 
             WHERE u.user_uuid = $1",
            user_uuid
        )
        .fetch_optional(pool)
        .await?
        {
            role_code = row.role_code;
        }

        if let Some(row) = query!(
            "SELECT user_name 
             FROM user_ms 
             WHERE user_uuid = $1",
            user_uuid
        )
        .fetch_optional(pool)
        .await?
        {
            user_name = row.user_name;
        }

        return Ok(UserDetails {
            user_uuid,
            role_code,
            division_title,
            division_code,
            user_name,
            user_id,
            is_authenticated,
        });
    }

    Err("Authentication failed".into())
}


pub async fn change_password(pool: &PgPool, password: ChangePassword, user_uuid: &str) -> Result<(), Box<dyn Error>> {
    let db_password = query!(
        "SELECT user_password 
         FROM user_ms 
         WHERE user_uuid = $1",
        user_uuid.to_string()
    )
    .fetch_one(pool)
    .await?;

    if password.new_password.len() < 8 {
        return Err("Password should be at least 8 characters long".into());
    }

    let decoded_password = general_purpose::STANDARD.decode(db_password.user_password)?;

    if verify(&password.old_password, str::from_utf8(&decoded_password)?)? == false {
        return Err("Old password is incorrect".into());
    }

    if verify(&password.new_password, str::from_utf8(&decoded_password)?)? == true {
        return Err("New password must be different from the old password".into());
    }

    let new_hashed_password = hash(password.new_password, DEFAULT_COST).unwrap();
    let new_hashed_password_str = general_purpose::STANDARD.encode(new_hashed_password.as_bytes());

    query!(
        "UPDATE user_ms 
         SET user_password = $1 
         WHERE user_uuid = $2",
        new_hashed_password_str,
        user_uuid
    )
    .execute(pool)
    .await?;

    Ok(())
}