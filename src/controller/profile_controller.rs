use actix_web::{web, HttpResponse, HttpRequest, Responder};
use base64::decode;
use jsonwebtoken::{decode as jwt_decode, Algorithm, DecodingKey, Validation};
use rand::{rngs::OsRng, RngCore as _};
use ring::{aead::{self, Aad, LessSafeKey, Nonce, Tag, UnboundKey, AES_256_GCM}, error::Unspecified};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use std::error::Error;
use crate::{models::profile::Profile, service::profile_service, utils::{crypto::{decrypt_token, get_key}, token::validate_token}};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: i64,
    user_uuid: String,
    user_name: String,
    role_code: String,
    division_title: String,
    division_code: String,
    sub: String,
    exp: i64,
}

pub async fn my_profile(req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, Box<dyn Error>> {
    let claims = match validate_token(&req).await {
        Ok(claims) => claims,
        Err(response) => return Ok(response),
    };

    let user_uuid = claims.user_uuid;

    match profile_service::my_profile(&user_uuid, &pool).await {
        Ok(profile) => Ok(HttpResponse::Ok().json(profile)),
        Err(e) => {
            eprintln!("Error fetching profile: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}