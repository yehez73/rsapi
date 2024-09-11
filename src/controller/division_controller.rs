use std::error::Error;

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use crate::{service::division_service, utils::token::validate_token};

pub async fn getall_division(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match division_service::getall_division(pool.get_ref()).await {
        Ok(divisions) => HttpResponse::Ok().json(divisions),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_specific_division(pool: web::Data<sqlx::PgPool>, id: web::Path<uuid::Uuid>) -> impl Responder {
    match division_service::get_specific_division(pool.get_ref(), id.into_inner()).await {
        Ok(division) => HttpResponse::Ok().json(division),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn add_division(pool: web::Data<sqlx::PgPool>, division: web::Json<crate::models::division::Division>, req: HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let claims = match validate_token(&req).await {
        Ok(claims) => claims,
        Err(response) => return Ok(response),
    };

    let user_uuid = claims.user_uuid;

    match division_service::add_division(pool.get_ref(), division.into_inner(), &user_uuid).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Berhasil menambahkan divisi!",
            "status": true
        }))),
        Err(e) => {
            eprintln!("Gagal menambahkan divisi: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn update_division(pool: web::Data<sqlx::PgPool>, division: web::Json<crate::models::division::Division>, id: web::Path<uuid::Uuid>, req: HttpRequest) -> Result<HttpResponse, Box<dyn Error>>{
    let claims = match validate_token(&req).await {
        Ok(claims) => claims,
        Err(response) => return Ok(response),
    };

    let user_uuid = claims.user_uuid;

    match division_service::update_division(pool.get_ref(), division.into_inner(), id.into_inner(), &user_uuid).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Berhasil mengedit divisi!",
            "status": true
        }))),
        Err(e) => {
            eprintln!("Gagal mengedit divisi: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn delete_division(pool: web::Data<sqlx::PgPool>, id: web::Path<uuid::Uuid>, req: HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let claims = match validate_token(&req).await {
        Ok(claims) => claims,
        Err(response) => return Ok(response),
    };

    let user_uuid = claims.user_uuid;

    match division_service::delete_division(pool.get_ref(), id.into_inner(), &user_uuid).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Berhasil menghapus divisi!",
            "status": true
        }))),
        Err(e) => {
            eprintln!("Gagal menghapus divisi: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}