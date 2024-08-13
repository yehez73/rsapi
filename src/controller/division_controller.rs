use actix_web::{web, Responder, HttpResponse};
use crate::service::division_service;

pub async fn getall_division(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match division_service::getall_division(pool.get_ref()).await {
        Ok(divisions) => HttpResponse::Ok().json(divisions),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn add_division(pool: web::Data<sqlx::PgPool>, division: web::Json<crate::models::division::Division>) -> impl Responder {
    match division_service::add_division(pool.get_ref(), division.into_inner()).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update_division(pool: web::Data<sqlx::PgPool>, division: web::Json<crate::models::division::Division>, id: web::Path<uuid::Uuid>) -> impl Responder {
    match division_service::update_division(pool.get_ref(), division.into_inner(), id.into_inner()).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_division(pool: web::Data<sqlx::PgPool>, id: web::Path<uuid::Uuid>) -> impl Responder {
    match division_service::delete_division(pool.get_ref(), id.into_inner()).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}