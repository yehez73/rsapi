use actix_web::{web, HttpResponse, Responder};
use crate::service::user_service;
use crate::models::user::User;

pub async fn getall_user() -> impl Responder {
    match user_service::getall_user().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_user(user: web::Json<User>) -> impl Responder {
    match user_service::create_user(user.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_user(id: web::Path<i32>) -> impl Responder {
    match user_service::get_user(*id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_user(id: web::Path<i32>, user: web::Json<User>) -> impl Responder {
    match user_service::update_user(*id, user.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_user(id: web::Path<i32>) -> impl Responder {
    match user_service::delete_user(*id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}