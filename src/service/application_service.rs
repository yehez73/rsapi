use crate::models::application::Application;
use actix_web::{Error, HttpResponse};
use sqlx::{query, PgPool};
use anyhow::Result;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use log::error;

pub async fn getall_application(pool: &PgPool) -> Result<Vec<Application>, sqlx::Error> {
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

    let application = sqlx::query_as::<_, Application>(query).fetch_all(pool).await?;

    Ok(application)
}

pub async fn add_application(pool: &PgPool, application: Application) -> Result<HttpResponse, Error> {
    let username = "admin";

    // Check if application with the same title or code already exists
    if let Some(_row) = query!(
        "SELECT application_id FROM application_ms WHERE (application_title = $1 OR application_code = $2) AND deleted_at IS NULL",
        application.application_title,
        application.application_code
    )
    .fetch_optional(pool)
    .await
    .unwrap()
    {
        // Duplicate found, return error
        error!("Application with the same title or code already exists");
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

    // Generate a application_id by combining the timestamp with a byte from the UUID
    let application_id = format!("{}{}", current_timestamp_micros, unique_uuid_str.as_bytes()[0])
        .parse::<i64>()
        .unwrap();

    // Insert the new application into the database
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
        unique_uuid_str,
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

pub async fn update_application(pool: &PgPool, application: Application, id: Uuid) -> Result<HttpResponse, Error> {
    let username = "admin";

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

pub async fn delete_application(pool: &PgPool, id: Uuid) -> Result<HttpResponse, Error> {
    let username = "admin";

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