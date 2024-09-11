use std::error::Error;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use crate::{models::role::Role, service::role_service, utils::token::validate_token};

pub async fn getall_role(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match role_service::getall_role(pool.get_ref()).await {
        Ok(role) => HttpResponse::Ok().json(role),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_specific_role(pool: web::Data<sqlx::PgPool>, id: web::Path<uuid::Uuid>) -> impl Responder {
    match role_service::get_specific_role(pool.get_ref(), id.into_inner()).await {
        Ok(role) => HttpResponse::Ok().json(role),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn add_role(pool: web::Data<sqlx::PgPool>, role: web::Json<Role>, req: HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let claims = match validate_token(&req).await {
        Ok(claims) => claims,
        Err(response) => return Ok(response),
    };

    let user_uuid = claims.user_uuid;
    
    match role_service::add_role(pool.get_ref(), role.into_inner(), &user_uuid).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Berhasil Menambahkan Role!",
            "status": true
        }))),
        Err(e) => {
            eprintln!("Gagal Menambahkan Role: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn update_role(pool: web::Data<sqlx::PgPool>, role: web::Json<Role>, id: web::Path<uuid::Uuid>, req: HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let claims = match validate_token(&req).await {
        Ok(claims) => claims,
        Err(response) => return Ok(response),
    };

    let user_uuid = claims.user_uuid;

    match role_service::update_role(pool.get_ref(), role.into_inner(), id.into_inner(), &user_uuid).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Berhasil Mengedit Role!",
            "status": true
        }))),
        Err(e) => {
            eprintln!("Gagal Mengedit Role: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn delete_role(pool: web::Data<sqlx::PgPool>, id: web::Path<uuid::Uuid>, req: HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let claims = match validate_token(&req).await {
        Ok(claims) => claims,
        Err(response) => return Ok(response),
    };

    let user_uuid = claims.user_uuid;

    match role_service::delete_role(pool.get_ref(), id.into_inner(), &user_uuid).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Berhasil Menghapus Role!",
            "status": true
        }))),
        Err(e) => {
            eprintln!("Gagal Menghapus Role: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}