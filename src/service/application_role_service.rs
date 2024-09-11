use crate::models::application_role::{AddApplicationRole, ApplicationRole, ListAppRole};
use actix_web::HttpResponse;
use sqlx::{query, query_as, PgPool};
use anyhow::Result;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

use super::user_service::get_username_by_id;

pub async fn getall_application_role(pool: &PgPool) -> Result<Vec<ApplicationRole>, sqlx::Error> {
    let query = r#"
        SELECT 
            ar.application_role_uuid, 
            a.application_title, 
            r.role_title, 
            ar.application_id, 
            ar.role_id, 
            ar.created_by, 
            ar.created_at, 
            ar.updated_by, 
            ar.updated_at, 
            ar.deleted_by, 
            ar.deleted_at 
        FROM application_role_ms ar 
        JOIN application_ms a ON ar.application_id = a.application_id 
        JOIN role_ms r ON ar.role_id = r.role_id 
        WHERE ar.deleted_at IS NULL
    "#;

    let application_role = sqlx::query_as::<_, ApplicationRole>(query).fetch_all(pool).await?;

    Ok(application_role)
}

pub async fn get_specific_application_role(pool: &PgPool, id: String) -> Result<ApplicationRole, sqlx::Error> {
    let query = r#"
        SELECT 
            ar.application_role_uuid, 
            a.application_title, 
            r.role_title, 
            ar.application_id, 
            ar.role_id, 
            ar.created_by, 
            ar.created_at, 
            ar.updated_by, 
            ar.updated_at, 
            ar.deleted_by, 
            ar.deleted_at 
        FROM application_role_ms ar 
        JOIN application_ms a ON ar.application_id = a.application_id 
        JOIN role_ms r ON ar.role_id = r.role_id 
        WHERE ar.application_role_uuid = $1 
        AND ar.deleted_at IS NULL
    "#;

    let application_role = query_as::<_, ApplicationRole>(query)
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(application_role)
}

pub async fn list_application_role_by_id(pool: &PgPool, id: String) -> Result<Vec<ListAppRole>, sqlx::Error> {
    let query = r#"
        SELECT 
            r.role_uuid, 
            r.role_title 
        FROM application_role_ms ar 
        JOIN application_ms a 
        ON ar.application_id = a.application_id 
        JOIN role_ms r 
        ON ar.role_id = r.role_id 
        WHERE a.application_uuid = $1 
        AND ar.deleted_at IS NULL
    "#;

    let application_role = query_as::<_, ListAppRole>(query)
        .bind(id)
        .fetch_all(pool)
        .await?;

    Ok(application_role)
}

pub async fn add_application_role(pool: &PgPool, application_role: AddApplicationRole, id: &String) -> Result<HttpResponse> {
    let username = get_username_by_id(&pool, &id).await.map_err(|e| anyhow::Error::msg(e.to_string()))?;

    let current_offset_datetime = time::OffsetDateTime::now_local().unwrap_or_else(|err| {
        eprintln!("Error getting current time: {}", err);
        OffsetDateTime::now_local().unwrap()
    });

    let current_timestamp_micros = current_offset_datetime.unix_timestamp_nanos() / 1000;
    let unique_uuid = Uuid::new_v4();
    let app_role_id_string = format!("{}{}", current_timestamp_micros, unique_uuid.as_bytes()[0]);
    let app_role_id_string = format!("{:0>18}", &app_role_id_string[..18]);

    let app_role_id = app_role_id_string.parse::<i64>().unwrap();

    let role_id = query!(
        r#"
        SELECT role_id FROM role_ms WHERE role_uuid = $1 AND deleted_at IS NULL
        "#,
        application_role.role_uuid.to_string()
    )
    .fetch_one(pool)
    .await
    .map_err(|err| actix_web::error::ErrorInternalServerError(err)).unwrap()
    .role_id;

    let application_id = query!(
        r#"
        SELECT application_id FROM application_ms WHERE application_uuid = $1 AND deleted_at IS NULL
        "#,
        application_role.application_uuid.to_string()
    )
    .fetch_one(pool)
    .await
    .map_err(|err| actix_web::error::ErrorInternalServerError(err)).unwrap()
    .application_id;

    query!(
        r#"
        INSERT INTO application_role_ms (application_role_id, application_role_uuid, application_id, role_id, created_by)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        app_role_id,
        unique_uuid.to_string(),
        application_id,
        role_id,
        username,
    )
    .execute(pool)
    .await?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn update_application_role(pool: &PgPool, id: String, application_role: AddApplicationRole, user_uuid: &String) -> Result<HttpResponse> {
    let username = get_username_by_id(&pool, &user_uuid).await.unwrap();

    let role_id = query!(
        r#"
        SELECT role_id FROM role_ms WHERE role_uuid = $1 AND deleted_at IS NULL
        "#,
        application_role.role_uuid.to_string()
    )
    .fetch_one(pool)
    .await
    .map_err(|err| actix_web::error::ErrorInternalServerError(err)).unwrap()
    .role_id;

    let application_id = query!(
        r#"
        SELECT application_id FROM application_ms WHERE application_uuid = $1 AND deleted_at IS NULL
        "#,
        application_role.application_uuid.to_string()
    )
    .fetch_one(pool)
    .await
    .map_err(|err| actix_web::error::ErrorInternalServerError(err)).unwrap()
    .application_id;

    let new_pdt = {
        let now = time::OffsetDateTime::now_utc();
        PrimitiveDateTime::new(now.date(), now.time())
    };

    query!(
        r#"
        UPDATE application_role_ms 
        SET application_id = $1, role_id = $2, updated_by = $3, updated_at = $4
        WHERE application_role_uuid = $5
        "#,
        application_id,
        role_id,
        username,
        new_pdt, 
        id
    )
    .execute(pool)
    .await?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_application_role(pool: &PgPool, id: String, user_uuid: &String) -> Result<HttpResponse> {
    let username = get_username_by_id(&pool, &user_uuid).await.unwrap();

    let new_pdt = {
        let now = time::OffsetDateTime::now_utc();
        PrimitiveDateTime::new(now.date(), now.time())
    };

    query!(
        r#"
        UPDATE application_role_ms 
        SET deleted_by = $1, deleted_at = $2
        WHERE application_role_uuid = $3
        "#,
        username,
        new_pdt,
        id
    )
    .execute(pool)
    .await?;

    Ok(HttpResponse::Ok().finish())
}