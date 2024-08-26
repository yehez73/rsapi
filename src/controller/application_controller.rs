use actix_web::{web, Responder, HttpResponse};
use crate::{models::application::Application, service::application_service};

pub async fn getall_application(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match application_service::getall_application(pool.get_ref()).await {
        Ok(application) => HttpResponse::Ok().json(application),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn add_application(pool: web::Data<sqlx::PgPool>, application: web::Json<Application>) -> impl Responder {
    match application_service::add_application(pool.get_ref(), application.into_inner()).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update_application(pool: web::Data<sqlx::PgPool>, application: web::Json<Application>, id: web::Path<uuid::Uuid>) -> impl Responder {
    match application_service::update_application(pool.get_ref(), application.into_inner(), id.into_inner()).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_application(pool: web::Data<sqlx::PgPool>, id: web::Path<uuid::Uuid>) -> impl Responder {
    match application_service::delete_application(pool.get_ref(), id.into_inner()).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}