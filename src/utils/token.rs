use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use actix_web::{HttpRequest, HttpResponse};
use jsonwebtoken::{decode as jwt_decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use crate::utils::crypto::decrypt_token;
use crate::utils::crypto::get_key;

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

pub struct InvalidTokenStore {
    invalid_tokens: Arc<Mutex<HashSet<String>>>,
}

impl InvalidTokenStore {
    pub fn new() -> Self {
        Self {
            invalid_tokens: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn invalidate_token(&self, token: String) {
        let mut tokens = self.invalid_tokens.lock().unwrap();
        tokens.insert(token);
    }

    pub fn is_token_invalid(&self, token: &str) -> bool {
        let tokens = self.invalid_tokens.lock().unwrap();
        tokens.contains(token)
    }
}

lazy_static::lazy_static! {
    pub static ref TOKEN_STORE: InvalidTokenStore = InvalidTokenStore::new();
}

pub async fn validate_token(req: &HttpRequest) -> Result<Claims, HttpResponse> {
    let token_string = req.headers().get("Authorization")
        .map(|h| h.to_str().unwrap_or("").to_string())
        .unwrap_or_default();

    if token_string.is_empty() {
        return Err(HttpResponse::Unauthorized().json(json!({
            "code": 401,
            "message": "Token tidak ditemukan!",
            "status": false
        })));
    }

    if !token_string.starts_with("Bearer ") {
        return Err(HttpResponse::Unauthorized().json(json!({
            "code": 401,
            "message": "Bearer tidak ditemukan!",
            "status": false
        })));
    }

    let token_only = token_string.trim_start_matches("Bearer ").to_string();

    let decrypted = decrypt_token(&token_only, &get_key()).map_err(|e| {
        eprintln!("Error decrypting token: {}", e);
        HttpResponse::Unauthorized().json(json!({
            "code": 401,
            "message": "Token Decryption Failed!",
            "status": false
        }))
    })?;

    let validation = Validation::new(Algorithm::HS256);
    let decoding_key = &DecodingKey::from_secret("secret".as_ref());

    let token_data = jwt_decode::<Claims>(&decrypted, decoding_key, &validation).map_err(|e| {
        eprintln!("Error verifying token: {}", e);
        HttpResponse::Unauthorized().json(json!({
            "code": 401,
            "message": "Token Verification Failed!",
            "status": false
        }))
    })?;

    Ok(token_data.claims)
}