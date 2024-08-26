use crate::models::user_application_role::{self, UserApplicationRole};
use actix_web::{Error, HttpResponse};
use sqlx::{query, query_as, PgPool};
use anyhow::Result;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use log::{error};

pub async fn getall_userapplicationrole(pool: &PgPool) -> Result<Vec<UserApplicationRole>, sqlx::Error> {
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
            uar.updated_at
        FROM user_ms u
        INNER JOIN user_application_role_ms uar ON u.user_id = uar.user_id
        INNER JOIN application_role_ms ar ON uar.application_role_id = ar.application_role_id
        INNER JOIN application_ms a ON ar.application_id = a.application_id
        INNER JOIN role_ms r ON ar.role_id = r.role_id
        INNER JOIN division_ms d ON uar.division_id = d.division_id
        INNER JOIN personal_data_ms pdm ON u.user_id = pdm.user_id
        WHERE uar.deleted_at IS NULL
    "#;

    let user_application_role = sqlx::query_as::<_, UserApplicationRole>(query).fetch_all(pool).await?;

    Ok(user_application_role)
}
