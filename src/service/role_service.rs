use crate::models::role::{GetRole, Role};
use actix_web::HttpResponse;
use sqlx::{query, PgPool};
use anyhow::Result;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use log::error;

use super::user_service::get_username_by_id;

pub async fn getall_role(pool: &PgPool) -> Result<Vec<GetRole>, sqlx::Error> {
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

    let role = sqlx::query_as::<_, GetRole>(query).fetch_all(pool).await?;

    Ok(role)
}

pub async fn get_specific_role(pool: &PgPool, id: Uuid) -> Result<GetRole, sqlx::Error> {
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
        WHERE role_uuid = $1 AND deleted_at IS NULL
    "#;

    let role = sqlx::query_as::<_, GetRole>(query)
        .bind(id.to_string())
        .fetch_one(pool)
        .await?;

    Ok(role)
}

pub async fn add_role(pool: &PgPool, role: Role, user_uuid: &str) -> Result<HttpResponse> {
    let username = get_username_by_id(pool, &user_uuid).await.map_err(|e| anyhow::Error::msg(e.to_string()))?;

    if let Some(_row) = query!(
        "SELECT role_id FROM role_ms WHERE (role_title = $1 OR role_code = $2) AND deleted_at IS NULL",
        role.role_title,
        role.role_code
    )
    .fetch_optional(pool)
    .await
    .unwrap()
    {
        error!("Role with the same title or code already exists");
        return Ok(HttpResponse::BadRequest().finish());
    }

    let current_offset_datetime = time::OffsetDateTime::now_local().unwrap_or_else(|err| {
        eprintln!("Error getting current time: {}", err);
        OffsetDateTime::now_local().unwrap()
    });
    let current_timestamp_micros = current_offset_datetime.unix_timestamp_nanos() / 1000;

    let unique_uuid = Uuid::new_v4();

    let role_id_string = format!("{}{}", current_timestamp_micros, unique_uuid.as_bytes()[0]);
    let role_id_string = format!("{:0>18}", &role_id_string[..18]);

    let role_id = role_id_string.parse::<i64>().unwrap();

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
        unique_uuid.to_string(),
        role.role_code,
        role.role_title,
        username
    )
    .execute(pool)
    .await
    .unwrap();

    Ok(HttpResponse::Ok().finish())
}

pub async fn update_role(pool: &PgPool, role: Role, id: Uuid, user_uuid: &str) -> Result<HttpResponse> {
    let username = get_username_by_id(pool, &user_uuid).await.map_err(|e| anyhow::Error::msg(e.to_string()))?;

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

pub async fn delete_role(pool: &PgPool, id: Uuid, user_uuid: &str) -> Result<HttpResponse> {
    let username = get_username_by_id(pool, &user_uuid).await.map_err(|e| anyhow::Error::msg(e.to_string()))?;

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