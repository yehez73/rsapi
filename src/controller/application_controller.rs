use std::error::Error;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use crate::{models::application::Application, service::application_service, utils::token::validate_token};

pub async fn getall_application(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match application_service::getall_application(pool.get_ref()).await {
        Ok(application) => HttpResponse::Ok().json(application),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_specific_application(pool: web::Data<sqlx::PgPool>, id: web::Path<uuid::Uuid>) -> impl Responder {
    match application_service::get_specific_application(pool.get_ref(), id.clone()).await {
        Ok(application) => HttpResponse::Ok().json(application),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn add_application(pool: web::Data<sqlx::PgPool>, application: web::Json<Application>, req: HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let claims = match validate_token(&req).await {
        Ok(claims) => claims,
        Err(response) => return Ok(response),
    };

    let user_uuid = claims.user_uuid;

    match application_service::add_application(pool.get_ref(), application.into_inner(), &user_uuid).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Berhasil menambahkan aplikasi!",
            "status": true
        }))),
        Err(e) => {
            eprintln!("Gagal menambahkan aplikasi: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn update_application(pool: web::Data<sqlx::PgPool>, application: web::Json<Application>, req: HttpRequest, id: web::Path<uuid::Uuid>) -> Result<HttpResponse, Box<dyn Error>> {
    let claims = match validate_token(&req).await {
        Ok(claims) => claims,
        Err(response) => return Ok(response),
    };

    let user_uuid = claims.user_uuid;

    match application_service::update_application(pool.get_ref(), application.into_inner(), id.into_inner(), &user_uuid).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Berhasil mengedit aplikasi!",
            "status": true
        }))),
        Err(e) => {
            eprintln!("Gagal mengedit aplikasi: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub async fn delete_application(pool: web::Data<sqlx::PgPool>, id: web::Path<uuid::Uuid>, req: HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let claims = match validate_token(&req).await {
        Ok(claims) => claims,
        Err(response) => return Ok(response),
    };

    let user_uuid = claims.user_uuid;

    match application_service::delete_application(pool.get_ref(), id.into_inner(), &user_uuid).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Berhasil menghapus aplikasi!",
            "status": true
        }))),
        Err(e) => {
            eprintln!("Gagal menghapus aplikasi: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}