use actix_web::{web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::models::auth::Login;
use crate::service::auth_service::login as authenticate;
use jsonwebtoken::{encode, Header, EncodingKey};
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use ring::error::Unspecified;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn login(auth: web::Json<Login>, pool: web::Data<PgPool>) -> impl Responder {
    let login_data = auth.into_inner();

    match authenticate(&pool, &login_data).await {
        Ok(user_details) => {
            let claims = Claims {
                sub: login_data.user_email,
                exp: (chrono::Utc::now() + chrono::Duration::hours(2)).timestamp() as usize,
            };

            let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())) {
                Ok(t) => t,
                Err(_) => return HttpResponse::InternalServerError().body("Failed to create token"),
            };

            let encrypted_token = match encrypt_token(&token) {
                Ok(t) => t,
                Err(_) => return HttpResponse::InternalServerError().body("Failed to encrypt token"),
            };
            
            HttpResponse::Ok()
                .append_header(("Authorization", encrypted_token))
                .json(user_details)
        }
        Err(e) => {
            HttpResponse::Unauthorized().body(e.to_string())
        }
    }
}

fn encrypt_token(token: &str) -> Result<String, Unspecified> {
    let key = [0u8; 32];
    let nonce = Nonce::assume_unique_for_key([0u8; 12]);

    let unbound_key = UnboundKey::new(&AES_256_GCM, &key)?;
    let less_safe_key = LessSafeKey::new(unbound_key);

    let mut token_bytes = token.as_bytes().to_vec();
    less_safe_key.seal_in_place_append_tag(nonce, Aad::empty(), &mut token_bytes)?;

    Ok(base64::encode(&token_bytes))
}