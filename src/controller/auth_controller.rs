use std::collections::HashMap;
use std::error::Error;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use crate::models::auth::{ChangePassword, Login};
use crate::service::auth_service::{self, login as authenticate};
use crate::utils::token::{validate_token, Claims};
use jsonwebtoken::{decode as jwt_decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use crate::utils::crypto::{generate_key, get_key, decrypt_token, encrypt_token};

#[derive(Serialize)]
struct Response {
    code: u16,
    message: String,
    status: bool,
    token: Option<String>,
}

pub async fn login(auth: web::Json<Login>, pool: web::Data<PgPool>) -> impl Responder {
    let login_data = auth.into_inner();

    match authenticate(&pool, &login_data).await {
        Ok(user_details) => {
            let claims = Claims {
                user_id: user_details.user_id,
                user_uuid: user_details.user_uuid.clone(),
                user_name: user_details.user_name.clone(),
                role_code: user_details.role_code.clone(),
                division_title: user_details.division_title.clone(),
                division_code: user_details.division_code.clone(),
                standard_claims: {
                    let mut claims = HashMap::new();
                    claims.insert("sub".to_string(), serde_json::Value::String(login_data.user_email.clone()));
                    claims.insert("exp".to_string(), serde_json::Value::from((chrono::Utc::now() + chrono::Duration::hours(2)).timestamp() as i64));
                    claims
                },
            };

            let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())) {
                Ok(t) => t,
                Err(_) => {
                    return HttpResponse::InternalServerError().json(Response {
                        code: 500,
                        message: "Terjadi kesalahan pada internal server. Coba beberapa saat lagi!".to_string(),
                        status: false,
                        token: None,
                    });
                }
            };

            let encrypted_token = match encrypt_token(&token, (&generate_key)()) {
                Ok(t) => t,
                Err(_) => {
                    return HttpResponse::InternalServerError().json(Response {
                        code: 500,
                        message: "Terjadi kesalahan pada internal server. Coba beberapa saat lagi!".to_string(),
                        status: false,
                        token: None,
                    });
                }
            };
            
            HttpResponse::Ok().json(Response {
                code: 200,
                message: "Berhasil login".to_string(),
                status: true,
                token: Some(encrypted_token),
            })
        }
        Err(_) => {
            HttpResponse::Unauthorized().json(Response {
                code: 401,
                message: "Akun tidak ada atau password salah".to_string(),
                status: false,
                token: None,
            })
        }
    }
}

pub async fn change_password(pool: web::Data<sqlx::PgPool>, password: web::Json<ChangePassword>, req: HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let claims = match validate_token(&req).await {
        Ok(claims) => claims,
        Err(response) => return Ok(response),
    };

    let user_uuid = claims.user_uuid;
    println!("User UUID: {}", user_uuid);

    match auth_service::change_password(&pool, password.into_inner(), &user_uuid, ).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Password Changed Successfully, Please Re-Login!",
            "status": true
        }))),
        Err(e) => {
            eprintln!("Error fetching profile: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}