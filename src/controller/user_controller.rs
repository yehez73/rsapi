use actix_web::{web, Responder, HttpResponse};
use crate::models::user::Users;
use crate::service::user_service;

pub async fn getall_users(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match user_service::getall_users(pool.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// pub async fn create_user(user: web::Json<User>) -> impl Responder {
//     match user_service::create_user(user.into_inner()).await {
//         Ok(_) => HttpResponse::Created().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }

// pub async fn get_user(id: web::Path<i32>) -> impl Responder {
//     match user_service::get_user(*id).await {
//         Ok(user) => HttpResponse::Ok().json(user),
//         Err(_) => HttpResponse::NotFound().finish(),
//     }
// }

// pub async fn update_user(id: web::Path<i32>, user: web::Json<User>) -> impl Responder {
//     match user_service::update_user(*id, user.into_inner()).await {
//         Ok(_) => HttpResponse::Ok().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }

// pub async fn delete_user(id: web::Path<i32>) -> impl Responder {
//     match user_service::delete_user(*id).await {
//         Ok(_) => HttpResponse::Ok().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }