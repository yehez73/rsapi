use actix_web::{web, Responder, HttpResponse};
use crate::service::user_service;

pub async fn getall_users(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match user_service::getall_users(pool.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
