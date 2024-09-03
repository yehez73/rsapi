use crate::models::application_role::ApplicationRole;
use actix_web::{Error, HttpResponse};
use sqlx::{query, query_as, PgPool};
use anyhow::Result;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use log::error;

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