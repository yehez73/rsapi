use crate::models::user::{Register, Users, Gender};
use actix_web::HttpResponse;
use base64::{engine::general_purpose, Engine as _};
use bcrypt::{hash, DEFAULT_COST};
use chrono::NaiveDate;
use phonenumber::country::Id::ID;
use sqlx::{query, PgPool};
use uuid::Uuid;
use anyhow::{bail, Context, Result};

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

    let users = sqlx::query_as::<_, Users>(query).fetch_all(pool).await?;

    Ok(users)
}

pub async fn add_user(pool: &PgPool, user: Register, user_uuid: &str) -> Result<HttpResponse> {
    if user.user_password.len() < 8 {
        bail!("Password must be at least 8 characters long");
    }

    let unique_uuid = Uuid::new_v4();
    let current_offset_datetime = time::OffsetDateTime::now_local()
        .context("Error getting current time")?;

    let current_timestamp_micros = current_offset_datetime.unix_timestamp_nanos() / 1000;
    let user_id_string = format!("{}{}", current_timestamp_micros, unique_uuid.as_bytes()[0]);
    let user_id_string = format!("{:0>18}", &user_id_string[..18]);

    let user_id = user_id_string.parse::<i64>()
        .context("Failed to parse user ID")?;

    let hashed_password = hash(user.user_password, DEFAULT_COST)
        .context("Failed to hash the password")?;
    let hashed_password_str = general_purpose::STANDARD.encode(hashed_password.as_bytes());
    let username = get_username_by_id(pool, &user_uuid).await.map_err(|e| anyhow::Error::msg(e.to_string()))?;
    let unique_uuid_str = unique_uuid.to_string();

    query!(
        r#"
        INSERT INTO user_ms (user_id, user_uuid, user_name, user_email, user_password, created_by)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        user_id,
        unique_uuid_str,
        user.user_name,
        user.user_email,
        hashed_password_str,
        username,
    )
    .execute(pool)
    .await
    .context("Failed to insert new user into database")?;

    let role_id = query!(
        r#"
        SELECT role_id FROM role_ms WHERE role_uuid = $1 AND deleted_at IS NULL
        "#,
        user.applicationRole.role_uuid.to_string()
    )
    .fetch_one(pool)
    .await
    .context("Failed to fetch role ID from the database")?
    .role_id;

    let application_id = query!(
        r#"
        SELECT application_id FROM application_ms WHERE application_uuid = $1 AND deleted_at IS NULL
        "#,
        user.applicationRole.application_uuid.to_string()
    )
    .fetch_one(pool)
    .await
    .context("Failed to fetch application ID from the database")?
    .application_id;

    let division_id = query!(
        r#"
        SELECT division_id FROM division_ms WHERE division_uuid = $1 AND deleted_at IS NULL
        "#,
        user.applicationRole.division_uuid.to_string()
    )
    .fetch_one(pool)
    .await
    .context("Failed to fetch division ID from the database")?
    .division_id;

    let app_role_id = (current_timestamp_micros + unique_uuid.as_bytes()[1] as i128) as i64;

    let unique_uuid_str = Uuid::parse_str(&unique_uuid.to_string())
        .context("Failed to parse UUID")?;

    query!(r#"
        INSERT INTO application_role_ms (application_role_uuid, application_role_id, application_id, role_id, created_by)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        unique_uuid_str.to_string(),
        app_role_id,
        application_id,
        role_id,
        username,
    )
    .execute(pool)
    .await
    .context("Failed to insert new application role into the database")?;

    let application_role_id_result = query!(
        r#"
        SELECT application_role_id FROM application_role_ms WHERE application_id = $1 AND role_id = $2
        "#,
        application_id,
        role_id
    )
    .fetch_one(pool)
    .await;

    match application_role_id_result {
        Ok(record) => {
            let application_role_id = record.application_role_id;
            println!("New application role ID: {}", application_role_id);
            let _ = Ok::<HttpResponse, anyhow::Error>(HttpResponse::Ok().finish());
        }
        Err(sqlx::Error::RowNotFound) => {
            println!("No application role found for the given application_id and role_id");
            let _ = Ok::<HttpResponse, anyhow::Error>(HttpResponse::NotFound().finish());
        }
        Err(e) => {
            anyhow::bail!("Database query error: {:?}", e);
        }
    }

    let birthday_str = user.personal_birthday.format("%Y-%m-%d").to_string();
    let birthday_date = NaiveDate::parse_from_str(&birthday_str, "%Y-%m-%d")
        .context("Failed to parse date")?;

    let personal_number = phonenumber::parse(Some(ID), user.personal_phone.clone())
        .context("Failed to parse phone number")?;
    let personal_id = current_timestamp_micros + i128::from(unique_uuid.as_bytes()[2]);

    sqlx::query!(
        r#"
        INSERT INTO personal_data_ms (personal_id, personal_uuid, division_id, user_id, personal_name, personal_birthday, personal_gender, personal_phone, personal_address) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        personal_id as i64,
        unique_uuid_str as Uuid,
        division_id as i64,
        user_id as i64, 
        user.personal_name as String,
        birthday_date as NaiveDate,
        user.personal_gender.unwrap() as Gender,
        personal_number.national().to_string() as String,
        user.personal_address as String,
    )
    .execute(pool)
    .await
    .context("Failed to insert personal data into database")?;

    query!(
        r#"
        INSERT INTO user_application_role_ms (user_application_role_uuid, user_id, application_role_id, division_id, created_by)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        unique_uuid_str.to_string(),
        user_id,
        app_role_id,
        division_id,
        username,
    )
    .execute(pool)
    .await
    .context("Failed to insert user application role into database")?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn get_username_by_id(db_pool: &PgPool, user_uuid: &str) -> Result<String, actix_web::Error> {
    match sqlx::query!(
        "SELECT user_name FROM user_ms WHERE user_uuid = $1",
        user_uuid
    )
    .fetch_one(db_pool)
    .await {
        Ok(record) => Ok(record.user_name),
        Err(sqlx::Error::RowNotFound) => {
            eprintln!("No user found with UUID: {}", user_uuid);
            Err(actix_web::error::ErrorNotFound("User not found"))
        }
        Err(e) => {
            eprintln!("Database query error: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError("Internal server error"))
        }
    }
}

pub async fn get_specific_user(pool: &PgPool, id: Uuid) -> Result<Users, sqlx::Error> {
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
        WHERE uar.user_application_role_uuid = $1 AND uar.deleted_at IS NULL
    "#;

    let user = sqlx::query_as::<_, Users>(query)
        .bind(id.to_string())
        .fetch_one(pool)
        .await?;

    Ok(user)
}

