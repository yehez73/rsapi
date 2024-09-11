use crate::models::application::{Application, GetApplication};
use actix_web::HttpResponse;
use log::error;
use sqlx::{query, PgPool};
use anyhow::Result;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

use super::user_service::get_username_by_id;


pub async fn getall_application(pool: &PgPool) -> Result<Vec<GetApplication>> {
    let query = r#"
        SELECT 
            application_uuid, 
            application_order,
            application_code, 
            application_title,
            application_description, 
            application_show, 
            created_by, 
            created_at, 
            updated_by, 
            updated_at, 
            deleted_by, 
            deleted_at 
        FROM application_ms 
        WHERE deleted_at IS NULL
    "#;

    let applications = sqlx::query_as::<_, GetApplication>(query)
        .fetch_all(pool)
        .await?;

    Ok(applications)
}

pub async fn get_specific_application(pool: &PgPool, id: Uuid) -> Result<GetApplication> {
    let query = r#"
        SELECT 
            application_uuid, 
            application_order,
            application_code, 
            application_title,
            application_description, 
            application_show, 
            created_by, 
            created_at, 
            updated_by, 
            updated_at, 
            deleted_by, 
            deleted_at 
        FROM application_ms 
        WHERE application_uuid = $1 AND deleted_at IS NULL
    "#;

    let application = sqlx::query_as::<_, GetApplication>(query)
        .bind(id.to_string())
        .fetch_one(pool)
        .await?;

    Ok(application)
}

pub async fn add_application(pool: &PgPool, application: Application, user_uuid: &str) -> Result<HttpResponse> {
    let username = get_username_by_id(pool, &user_uuid).await.map_err(|e| anyhow::Error::msg(e.to_string()))?;

    if let Some(_row) = query!(
        "SELECT application_id FROM application_ms WHERE (application_title = $1 OR application_code = $2) AND deleted_at IS NULL",
        application.application_title,
        application.application_code
    )
    .fetch_optional(pool)
    .await
    .unwrap()
    {
        error!("Application with the same title or code already exists");
        return Ok(HttpResponse::BadRequest().finish());
    }

    let current_offset_datetime = time::OffsetDateTime::now_local().unwrap_or_else(|err| {
        eprintln!("Error getting current time: {}", err);
        OffsetDateTime::now_local().unwrap()
    });
    let current_timestamp_micros = current_offset_datetime.unix_timestamp_nanos() / 1000;

    let unique_uuid = Uuid::new_v4();

    let application_id_string = format!("{}{}", current_timestamp_micros, unique_uuid.as_bytes()[0]);
    let application_id_string = format!("{:0>18}", &application_id_string[..18]);

    let application_id = application_id_string.parse::<i64>().unwrap();
    
    query!(
        r#"
        INSERT INTO application_ms (
            application_id,
            application_uuid, 
            application_code, 
            application_title, 
            application_description, 
            created_by
        ) 
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        application_id,
        unique_uuid.to_string(),
        application.application_code,
        application.application_title,
        application.application_description,
        username,
    )
    .execute(pool)
    .await
    .unwrap();

    Ok(HttpResponse::Ok().finish())
}

pub async fn update_application(pool: &PgPool, application: Application, id: Uuid, user_uuid: &str) -> Result<HttpResponse> {
    let username = get_username_by_id(pool, &user_uuid).await.map_err(|e| anyhow::Error::msg(e.to_string()))?;

    let new_pdt = {
        let now = time::OffsetDateTime::now_utc();
        PrimitiveDateTime::new(now.date(), now.time())
    };

    query!(
        r#"
        UPDATE application_ms
        SET
            application_code = $1,
            application_title = $2,
            application_description = $3,
            updated_by = $4,
            updated_at = $5
        WHERE application_uuid = $6
        "#,
        application.application_code,
        application.application_title,
        application.application_description,
        username,
        new_pdt,
        id.to_string()
    )
    .execute(pool)
    .await
    .unwrap();

    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_application(pool: &PgPool, id: Uuid, user_uuid: &str) -> Result<HttpResponse> {
    let username = get_username_by_id(pool, &user_uuid).await.map_err(|e| anyhow::Error::msg(e.to_string()))?;

    let new_pdt = {
        let now = time::OffsetDateTime::now_utc();
        PrimitiveDateTime::new(now.date(), now.time())
    };

    query!(
        r#"
        UPDATE application_ms
        SET
            deleted_by = $1,
            deleted_at = $2
        WHERE application_uuid = $3
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