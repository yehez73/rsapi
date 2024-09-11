use std::error::Error;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;
use crate::{models::user::Register, service::user_service, utils::token::validate_token};

pub async fn getall_users(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match user_service::getall_users(pool.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_specific_user(pool: web::Data<sqlx::PgPool>, id: web::Path<Uuid>) -> impl Responder {
    match user_service::get_specific_user(pool.get_ref(), id.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn add_user(pool: web::Data<sqlx::PgPool>, user: web::Json<Register>, req: HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let claims = match validate_token(&req).await {
        Ok(claims) => claims,
        Err(response) => return Ok(response),
    };

    let user_uuid = claims.user_uuid;

    match user_service::add_user(&pool.get_ref(), user.into_inner(), &user_uuid).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Berhasil Menambahkan User!",
            "status": true
        }))),
        Err(e) => {
            eprintln!("Gagal Menambahkan User: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}