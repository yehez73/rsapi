use std::collections::HashMap;
use std::hash::Hash;
use actix_web::{web, Responder, HttpResponse};
use rand::RngCore as _;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::models::auth::Login;
use crate::service::auth_service::login as authenticate;
use jsonwebtoken::{encode, Header, EncodingKey};
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use ring::error::Unspecified;
use rand::rngs::OsRng;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i64,
    pub user_uuid: String,
    pub user_name: String,
    pub role_code: String,
    pub division_title: String,
    pub division_code: String,
    #[serde(flatten)]
    pub standard_claims: HashMap<String, serde_json::Value>,
}

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

            let encrypted_token = match encrypt_token(&token) {
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

fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

fn encrypt_token(token: &str) -> Result<String, Unspecified> {
    let key = generate_key(); // Generate a random key
    let nonce = Nonce::assume_unique_for_key(rand::random::<[u8; 12]>());

    let unbound_key = UnboundKey::new(&AES_256_GCM, &key)?;
    let less_safe_key = LessSafeKey::new(unbound_key);

    let mut token_bytes = token.as_bytes().to_vec();
    less_safe_key.seal_in_place_append_tag(nonce, Aad::empty(), &mut token_bytes)?;

    Ok(base64::encode(&token_bytes))
}