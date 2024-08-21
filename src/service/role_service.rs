use crate::models::role::{self, Role};
use actix_web::{Error, HttpResponse};
use sqlx::{query, query_as, PgPool};
use anyhow::Result;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use log::{error};

pub async fn getall_role(pool: &PgPool) -> Result<Vec<Role>, sqlx::Error> {
    let query = r#"
        SELECT 
            role_uuid, 
            role_order,
            role_code, 
            role_title, 
            role_show, 
            created_by, 
            created_at, 
            updated_by, 
            updated_at, 
            deleted_by, 
            deleted_at 
        FROM role_ms 
        WHERE deleted_at IS NULL
    "#;

    let divisions = sqlx::query_as::<_, Role>(query).fetch_all(pool).await?;

    Ok(divisions)
}

pub async fn add_role(pool: &PgPool, role: Role) -> Result<HttpResponse, Error> {
    let username = "admin";

    // Check if role with the same title or code already exists
    if let Some(row) = query!(
        "SELECT role_id FROM role_ms WHERE (role_title = $1 OR role_code = $2) AND deleted_at IS NULL",
        role.role_title,
        role.role_code
    )
    .fetch_optional(pool)
    .await
    .unwrap()
    {
        // Duplicate found, return error
        error!("Role with the same title or code already exists");
        return Ok(HttpResponse::BadRequest().finish());
    }

    // Get the current timestamp
    let current_offset_datetime = time::OffsetDateTime::now_local().unwrap_or_else(|err| {
        eprintln!("Error getting current time: {}", err);
        OffsetDateTime::now_local().unwrap()
    });
    let current_timestamp_micros = current_offset_datetime.unix_timestamp_nanos() / 1000;

    // Generate a unique UUID
    let unique_uuid_str = Uuid::new_v4().to_string();

    // Generate a role_id by combining the timestamp with a byte from the UUID
    let role_id = format!("{}{}", current_timestamp_micros, unique_uuid_str.as_bytes()[0])
        .parse::<i64>()
        .unwrap();

    // Insert the new role into the database
    query!(
        r#"
        INSERT INTO role_ms (
            role_id,
            role_uuid,
            role_code,
            role_title,
            created_by
        )
        VALUES ($1, $2, $3, $4, $5)
        "#,
        role_id,
        unique_uuid_str,
        role.role_code,
        role.role_title,
        username
    )
    .execute(pool)
    .await
    .unwrap();

    Ok(HttpResponse::Ok().finish())
}

pub async fn update_role(pool: &PgPool, role: Role, id: Uuid) -> Result<HttpResponse, Error> {
    let username = "admin";

    let new_pdt = {
        let now = time::OffsetDateTime::now_utc();
        PrimitiveDateTime::new(now.date(), now.time())
    };

    query!(
        r#"
        UPDATE role_ms
        SET
            role_code = $1,
            role_title = $2,
            updated_by = $3,
            updated_at = $4
        WHERE role_uuid = $5
        "#,
        role.role_code,
        role.role_title,
        username,
        new_pdt,
        id.to_string()
    )
    .execute(pool)
    .await
    .unwrap();

    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_role(pool: &PgPool, id: Uuid) -> Result<HttpResponse, Error> {
    let username = "admin";

    let new_pdt = {
        let now = time::OffsetDateTime::now_utc();
        PrimitiveDateTime::new(now.date(), now.time())
    };

    query!(
        r#"
        UPDATE role_ms
        SET
            deleted_by = $1,
            deleted_at = $2
        WHERE role_uuid = $3
        "#,
        username,
        new_pdt,
        id.to_string()
    )
    .execute(pool)
    .await
    .unwrap();

    Ok(HttpResponse::Ok().finish())
}