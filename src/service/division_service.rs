use crate::models::division::{Division, GetDivision};
use actix_web::HttpResponse;
use sqlx::{query, PgPool};
use anyhow::Result;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

use super::user_service::get_username_by_id;

pub async fn getall_division(pool: &PgPool) -> Result<Vec<GetDivision>, sqlx::Error> {
    let query = r#"
        SELECT 
            division_uuid, 
            division_order,
            division_code, 
            division_title, 
            division_show, 
            created_by, 
            created_at, 
            updated_by, 
            updated_at, 
            deleted_by, 
            deleted_at 
        FROM division_ms 
        WHERE deleted_at IS NULL
    "#;

    let divisions = sqlx::query_as::<_, GetDivision>(query).fetch_all(pool).await?;

    Ok(divisions)
}

pub async fn get_specific_division(pool: &PgPool, id: Uuid) -> Result<GetDivision, sqlx::Error> {
    let query = r#"
        SELECT 
            division_uuid, 
            division_order,
            division_code, 
            division_title, 
            division_show, 
            created_by, 
            created_at, 
            updated_by, 
            updated_at, 
            deleted_by, 
            deleted_at 
        FROM division_ms 
        WHERE division_uuid = $1
    "#;

    let division = sqlx::query_as::<_, GetDivision>(query)
        .bind(id.to_string())
        .fetch_one(pool)
        .await?;

    Ok(division)
}

pub async fn add_division(pool: &PgPool, division: Division, user_uuid: &str) -> Result<HttpResponse> {
    let username = get_username_by_id(pool, &user_uuid).await.map_err(|e| anyhow::Error::msg(e.to_string()))?;


    let current_offset_datetime = time::OffsetDateTime::now_local().unwrap_or_else(|err| {
        eprintln!("Error getting current time: {}", err);
        OffsetDateTime::now_local().unwrap()
    });

    let current_timestamp_micros = current_offset_datetime.unix_timestamp_nanos() / 1000;

    let unique_uuid = Uuid::new_v4();

    let division_id_string = format!("{}{}", current_timestamp_micros, unique_uuid.as_bytes()[0]);
    let division_id_string = format!("{:0>18}", &division_id_string[..18]);

    let division_id = division_id_string.parse::<i64>().unwrap();

    query!(
        r#"
        INSERT INTO division_ms (
            division_id,
            division_uuid,
            division_code,
            division_title,
            created_by
        )
        VALUES ($1, $2, $3, $4, $5)
        "#,
        division_id,
        unique_uuid.to_string(),
        division.division_code,
        division.division_title,
        username,
    )
    .execute(pool)
    .await
    .unwrap();

    Ok(HttpResponse::Ok().finish())
}

pub async fn update_division(pool: &PgPool, division: Division, id: Uuid, user_uuid: &str) -> Result<HttpResponse> {
    let username = get_username_by_id(pool, &user_uuid).await.map_err(|e| anyhow::Error::msg(e.to_string()))?;

    let new_pdt = {
        let now = time::OffsetDateTime::now_utc();
        PrimitiveDateTime::new(now.date(), now.time())
    };    

    query!(
        r#"
        UPDATE division_ms
        SET division_code = $1, division_title = $2, updated_by = $3, updated_at = $4
        WHERE division_uuid = $5
        "#,
        division.division_code,
        division.division_title,
        username,
        new_pdt,
        id.to_string(),
    )
    .execute(pool)
    .await
    .unwrap();

    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_division(pool: &PgPool, id: Uuid, user_uuid: &str) -> Result<HttpResponse> {
    let username = get_username_by_id(pool, &user_uuid).await.map_err(|e| anyhow::Error::msg(e.to_string()))?;

    let new_pdt = {
        let now = time::OffsetDateTime::now_utc();
        PrimitiveDateTime::new(now.date(), now.time())
    };

    query!(
        r#"
        UPDATE division_ms
        SET deleted_by = $1, deleted_at = $2
        WHERE division_uuid = $3
        "#,
        username,
        new_pdt,
        id.to_string(),
    )
    .execute(pool)
    .await
    .unwrap();

    Ok(HttpResponse::Ok().finish())
}