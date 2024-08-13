use crate::models::division::Division;
use actix_web::{Error, HttpResponse};
use sqlx::{query, query_as, PgPool};
use anyhow::Result;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

pub async fn getall_division(pool: &PgPool) -> Result<Vec<Division>, sqlx::Error> {
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

    let divisions = sqlx::query_as::<_, Division>(query).fetch_all(pool).await?;

    Ok(divisions)
}

pub async fn add_division(pool: &PgPool, division: Division) -> Result<HttpResponse, Error> {
    // let user_uuid = Uuid::new_v4();
    // let username = get_username_by_id(pool, user_uuid).await?;

    let username = "admin";

    let current_offset_datetime = time::OffsetDateTime::now_local().unwrap_or_else(|err| {
        eprintln!("Error getting current time: {}", err);
        OffsetDateTime::now_local().unwrap()
    });

    let current_timestamp_micros = current_offset_datetime.unix_timestamp_nanos() / 1000;

    let unique_uuid_str = Uuid::new_v4().to_string();

    let division_id = format!("{}{}", current_timestamp_micros, unique_uuid_str.as_bytes()[0])
        .parse::<i64>()
        .unwrap();

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
        unique_uuid_str,
        division.division_code,
        division.division_title,
        username,
    )
    .execute(pool)
    .await
    .unwrap();

    Ok(HttpResponse::Ok().finish())
}

pub async fn update_division(pool: &PgPool, division: Division, id: Uuid) -> Result<HttpResponse, Error> {
    let user_uuid = Uuid::new_v4().to_string();
    // let username = get_username_by_id(pool, &user_uuid).await?;
    let username = "admin";

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

pub async fn delete_division(pool: &PgPool, id: Uuid) -> Result<HttpResponse, Error> {
    let user_uuid = Uuid::new_v4().to_string();
    // let username = get_username_by_id(pool, &user_uuid).await?;
    let username = "admin";

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

// async fn get_username_by_id(db_pool: &PgPool, user_uuid: &str) -> Result<String, actix_web::Error> {
//     match sqlx::query!(
//         "SELECT user_name FROM user_ms WHERE user_uuid = $1",
//         user_uuid
//     )
//     .fetch_one(db_pool)
//     .await {
//         Ok(record) => Ok(record.user_name),
//         Err(sqlx::Error::RowNotFound) => {
//             eprintln!("No user found with UUID: {}", user_uuid);
//             Err(actix_web::error::ErrorNotFound("User not found"))
//         }
//         Err(e) => {
//             eprintln!("Database query error: {:?}", e);
//             Err(actix_web::error::ErrorInternalServerError("Internal server error"))
//         }
//     }
// }