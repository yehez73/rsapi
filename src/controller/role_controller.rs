use actix_web::{web, Responder, HttpResponse};
use crate::{models::role::Role, service::role_service};

pub async fn getall_role(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match role_service::getall_role(pool.get_ref()).await {
        Ok(role) => HttpResponse::Ok().json(role),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn add_role(pool: web::Data<sqlx::PgPool>, role: web::Json<Role>) -> impl Responder {
    match role_service::add_role(pool.get_ref(), role.into_inner()).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update_role(pool: web::Data<sqlx::PgPool>, role: web::Json<Role>, id: web::Path<uuid::Uuid>) -> impl Responder {
    match role_service::update_role(pool.get_ref(), role.into_inner(), id.into_inner()).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_role(pool: web::Data<sqlx::PgPool>, id: web::Path<uuid::Uuid>) -> impl Responder {
    match role_service::delete_role(pool.get_ref(), id.into_inner()).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}