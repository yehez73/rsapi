use crate::models::user::{Register, Users};
use actix_web::{Error, HttpResponse};
use base64::{engine::general_purpose, Engine as _};
use bcrypt::{hash, DEFAULT_COST};
use phonenumber::{country::Id::ID, NationalNumber};
use sqlx::{query, PgPool};
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn getall_users(pool: &PgPool) -> Result<Vec<Users>, sqlx::Error> {
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
            uar.updated_at,
            uar.deleted_by,
            uar.deleted_at
        FROM user_ms u 
        INNER JOIN user_application_role_ms uar ON u.user_id = uar.user_id 
        INNER JOIN application_role_ms ar ON uar.application_role_id = ar.application_role_id 
        INNER JOIN application_ms a ON ar.application_id = a.application_id 
        INNER JOIN role_ms r ON ar.role_id = r.role_id 
        INNER JOIN division_ms d ON uar.division_id = d.division_id 
        INNER JOIN personal_data_ms pdm ON u.user_id = pdm.user_id 
        WHERE uar.deleted_at IS NULL
    "#;

    let users = sqlx::query_as::<_, Users>(query).fetch_all(pool).await?;

    Ok(users)
}

pub async fn add_user(pool: &PgPool, user: Register) -> Result<HttpResponse, Error> {
    if user.user_password.len() < 8 {
        return Err(actix_web::error::ErrorBadRequest(
            "Password must be at least 8 characters",
        ));
    }

    let unique_uuid = Uuid::new_v4();
    let current_offset_datetime = time::OffsetDateTime::now_local();
    let current_offset_datetime = match current_offset_datetime {
        Ok(offset_datetime) => offset_datetime,
        Err(err) => {
            eprintln!("Error: {}", err);
            OffsetDateTime::now_local().unwrap()
        }
    };

    let current_timestamp_micros = current_offset_datetime.unix_timestamp_nanos() / 1000;
    let user_id = format!("{}{}", current_timestamp_micros, unique_uuid.as_bytes()[0])
        .parse::<i64>()
        .unwrap();
    let user_uuid = "5b9909a5-216e-4e20-95a3-8b9b0114b401";
    let hashed_password = hash(user.user_password, DEFAULT_COST).unwrap();
    let hashed_password_str = general_purpose::STANDARD.encode(hashed_password.as_bytes());
    let username = get_username_by_id(pool, user_uuid).await.unwrap();
    let unique_uuid_str = unique_uuid.to_string();

    // query!(
    //     r#"
    //     INSERT INTO user_ms (user_id, user_uuid, user_name, user_email, user_password, created_by)
    //     VALUES ($1, $2, $3, $4, $5, $6)
    //     "#,
    //     user_id,
    //     unique_uuid_str,
    //     user.user_name,
    //     user.user_email,
    //     hashed_password_str,
    //     username,
    // )
    // .execute(pool)
    // .await
    // .unwrap();

    println!("New user created with UUID: {}", unique_uuid_str);

    let role_id: i64 = query!(
        r#"
        SELECT role_id FROM role_ms WHERE role_uuid = $1 AND deleted_at IS NULL
        "#,
        user.application_role.role_uuid.to_string()
    )
    .fetch_one(pool)
    .await
    .unwrap()
    .role_id;

    let application_id: i64 = query!(
        r#"
        SELECT application_id FROM application_ms WHERE application_uuid = $1 AND deleted_at IS NULL
        "#,
        user.application_role.application_uuid.to_string()
    )
    .fetch_one(pool)
    .await
    .unwrap()
    .application_id;

    let division_id: i64 = query!(
        r#"
        SELECT division_id FROM division_ms WHERE division_uuid = $1 AND deleted_at IS NULL
        "#,
        user.application_role.division_uuid.to_string()
    )
    .fetch_one(pool)
    .await
    .unwrap()
    .division_id;

    let app_role_id = (current_timestamp_micros + unique_uuid.as_bytes()[1] as i128) as i64;

    // let unique_uuid_str = unique_uuid.to_string();
    // query!(r#"
    //     INSERT INTO application_role_ms (application_role_uuid, application_role_id, application_id, role_id, created_by)
    //     VALUES ($1, $2, $3, $4, $5)
    //     "#,
    //     unique_uuid_str,
    //     app_role_id,
    //     application_id,
    //     role_id,
    //     username,
    // )
    // .execute(pool)
    // .await
    // .unwrap();

    println!(
        "New application role created with UUID: {}",
        unique_uuid_str
    );

    let application_role_id_result = query!(
        r#"
        SELECT application_role_id FROM application_role_ms WHERE application_id = $1 AND role_id = $2
        "#,
        application_id,
        role_id
    )
    .fetch_one(pool)
    .await;

    match application_role_id_result {
        Ok(record) => {
            let application_role_id = record.application_role_id;
            println!("New application role ID: {}", application_role_id);
            Ok::<HttpResponse, Error>(HttpResponse::Ok().finish());
        }
        Err(sqlx::Error::RowNotFound) => {
            println!("No application role found for the given application_id and role_id");
            Ok::<HttpResponse, Error>(HttpResponse::NotFound().finish());
        }
        Err(e) => {
            println!("Database query error: {:?}", e);
            Ok::<HttpResponse, Error>(HttpResponse::InternalServerError().finish());
        }
    }

    let birthday = user.personal_birthday.format("%Y-%m-%d");
    let personal_number = phonenumber::parse(Some(ID), user.personal_phone.clone()).unwrap();

    query!(
        r#"
        INSERT INTO personal_data_ms (personal_id, personal_uuid, division_id, user_id, personal_name, personal_birthday, personal_gender, personal_phone, personal_address) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        personal_id = current_timestamp_micros + unique_uuid.as_bytes()[2] as i64,
        personal_uuid = unique_uuid_str,
        division_id = division_id,
        user_id = user_id,
        personal_name = user.personal_name,
        personal_birthday = user.personal_birthday,
        personal_gender = user.personal_gender,
        personal_phone = personal_number.national().to_string(),
        personal_address = user.personal_address,
    )
    .execute(pool)
    .await
    .unwrap();

    Ok(HttpResponse::Ok().finish())
}

async fn get_username_by_id(db_pool: &PgPool, user_uuid: &str) -> Result<String, sqlx::Error> {
    let username = sqlx::query!(
        "SELECT user_name FROM user_ms WHERE user_uuid = $1",
        user_uuid
    )
    .fetch_one(db_pool)
    .await?
    .user_name;
    Ok(username)
}
