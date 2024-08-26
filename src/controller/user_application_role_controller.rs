use actix_web::{web, Responder, HttpResponse};
use crate::{models::user_application_role, service::user_application_role_service};

pub async fn getall_userapplicationrole(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match user_application_role_service::getall_userapplicationrole(pool.get_ref()).await {
        Ok(user_application_role) => HttpResponse::Ok().json(user_application_role),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// pub async fn update_division(pool: web::Data<sqlx::PgPool>, division: web::Json<crate::models::division::Division>, id: web::Path<uuid::Uuid>) -> impl Responder {
//     match division_service::update_division(pool.get_ref(), division.into_inner(), id.into_inner()).await {
//         Ok(response) => response,
//         Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
//     }
// }

// pub async fn delete_division(pool: web::Data<sqlx::PgPool>, id: web::Path<uuid::Uuid>) -> impl Responder {
//     match division_service::delete_division(pool.get_ref(), id.into_inner()).await {
//         Ok(response) => response,
//         Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
//     }
// }