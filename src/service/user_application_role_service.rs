use crate::{service::user_service::get_username_by_id, models::user_application_role::UserApplicationRole};
use actix_web::{web, HttpResponse};
use sqlx::{query, PgPool};
use anyhow::{Context, Result};
use time::PrimitiveDateTime;
use uuid::Uuid;

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

pub async fn delete_user_application_role(pool: &PgPool, id: web::Path<Uuid>, user_uuid: &str ) -> Result<HttpResponse>{
    let username = get_username_by_id(pool, &user_uuid).await.map_err(|e| anyhow::Error::msg(e.to_string()))?;

    let new_pdt = {
        let now = time::OffsetDateTime::now_utc();
        PrimitiveDateTime::new(now.date(), now.time())
    };

    query!(
        r#"
        UPDATE user_application_role_ms
        SET deleted_by = $1, deleted_at = $2
        WHERE user_application_role_uuid = $3 AND deleted_at IS NULL
        "#,
        username,
        new_pdt,
        id.to_string()
    )
    .execute(pool)
    .await
    .context("Failed to delete user application role")?;

    let user_id = query!(
        r#"
        SELECT user_id
        FROM user_application_role_ms
        WHERE user_application_role_uuid = $1
        "#,
        id.to_string()
    )
    .fetch_one(pool)
    .await
    .context("Failed to get user id")?
    .user_id;

    query!(
        r#"
        UPDATE user_ms
        SET deleted_by = $1, deleted_at = $2
        WHERE user_id = $3 AND deleted_at IS NULL
        "#,
        username,
        new_pdt,
        user_id
    )
    .execute(pool)
    .await
    .context("Failed to delete user")?;

    Ok(HttpResponse::Ok().finish())
}