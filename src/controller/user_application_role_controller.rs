use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;
use crate::{service::user_application_role_service, utils::token::validate_token};
use std::error::Error;

pub async fn getall_userapplicationrole(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match user_application_role_service::getall_userapplicationrole(pool.get_ref()).await {
        Ok(user_application_role) => HttpResponse::Ok().json(user_application_role),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_user_application_role(pool: web::Data<sqlx::PgPool>, id: web::Path<Uuid>, req: HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let claims = match validate_token(&req).await {
        Ok(claims) => claims,
        Err(response) => return Ok(response),
    };

    let user_uuid = claims.user_uuid;

    match user_application_role_service::delete_user_application_role(pool.get_ref(), id, &user_uuid).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Berhasil Menghapus User!",
            "status": true
        }))),
        Err(e) => {
            eprintln!("Gagal Menghapus User: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}