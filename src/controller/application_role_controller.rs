use std::error::Error;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use crate::{models::application_role::AddApplicationRole, service::application_role_service, utils::token::validate_token};

pub async fn getall_application_role(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match application_role_service::getall_application_role(pool.get_ref()).await {
        Ok(application_role) => HttpResponse::Ok().json(application_role),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_specific_application_role(pool: web::Data<sqlx::PgPool>, id: web::Path<uuid::Uuid>) -> impl Responder {
    match application_role_service::get_specific_application_role(pool.get_ref(), id.to_string()).await {
        Ok(application_role) => HttpResponse::Ok().json(application_role),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn list_application_role_by_id(pool: web::Data<sqlx::PgPool>, id: web::Path<uuid::Uuid>) -> impl Responder {
    match application_role_service::list_application_role_by_id(pool.get_ref(), id.to_string()).await {
        Ok(application_role) => HttpResponse::Ok().json(application_role),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn add_application_role(pool: web::Data<sqlx::PgPool>, application_role: web::Json<AddApplicationRole>, req: HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let claims = match validate_token(&req).await {
        Ok(claims) => claims,
        Err(response) => return Ok(response),
    };

    let user_uuid = claims.user_uuid;
    
    match application_role_service::add_application_role(pool.get_ref(), application_role.into_inner(), &user_uuid).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Berhasil menambahkan aplikasi role!",
            "status": true
        }))),
        Err(e) => {
            eprintln!("Gagal menambahkan aplikasi role: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn update_application_role(pool: web::Data<sqlx::PgPool>, id: web::Path<uuid::Uuid>, application_role: web::Json<AddApplicationRole>, req: HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let claims = match validate_token(&req).await {
        Ok(claims) => claims,
        Err(response) => return Ok(response),
    };

    let user_uuid = claims.user_uuid;

    match application_role_service::update_application_role(pool.get_ref(), id.to_string(), application_role.into_inner(), &user_uuid).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Berhasil mengubah aplikasi role!",
            "status": true
        }))),
        Err(e) => {
            eprintln!("Gagal mengubah aplikasi role: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn delete_application_role(pool: web::Data<sqlx::PgPool>, id: web::Path<uuid::Uuid>, req: HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let claims = match validate_token(&req).await {
        Ok(claims) => claims,
        Err(response) => return Ok(response),
    };

    let user_uuid = claims.user_uuid;

    match application_role_service::delete_application_role(pool.get_ref(), id.to_string(), &user_uuid).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Berhasil menghapus aplikasi role!",
            "status": true
        }))),
        Err(e) => {
            eprintln!("Gagal menghapus aplikasi role: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}