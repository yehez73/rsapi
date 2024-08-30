use actix_web::{web, HttpResponse, HttpRequest, Responder};
use base64::decode;
use rand::{rngs::OsRng, RngCore as _};
use ring::{aead::{self, Aad, LessSafeKey, Nonce, Tag, UnboundKey, AES_256_GCM}, error::Unspecified};
use serde_json::json;
use sqlx::PgPool;
use std::error::Error;
use crate::{models::profile::Profile, service::profile_service};

pub async fn my_profile(req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, Box<dyn Error>> {
    let token_string = req.headers().get("Authorization")
        .map(|h| h.to_str().unwrap_or("").to_string())
        .unwrap_or_default();

    println!("Token: {}", token_string);

    let secret_key = "your-secret-key";

    if token_string.is_empty() {
        return Ok(HttpResponse::Unauthorized().json(json!({
            "code": 401,
            "message": "Token tidak ditemukan!",
            "status": false
        })));
    }

    // Periksa apakah token_string mengandung "Bearer "
    if !token_string.starts_with("Bearer ") {
        return Ok(HttpResponse::Unauthorized().json(json!({
            "code": 401,
            "message": "Token tidak valid!",
            "status": false
        })));
    }

    // Hapus "Bearer " dari token_string
    let token_only = token_string.trim_start_matches("Bearer ").to_string();

    println!("Token only: {}", token_only);
    println!("secret_key: {}", secret_key);

    // Dekripsi token JWE
    let decrypted = match decrypt_jwe(&token_only) {
        Ok(d) => d,
        Err(_) => {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "code": 401,
                "message": "Token tidak valid!",
                "status": false
            })));
        }
    };

    // Untuk demonstrasi, kita anggap decrypted token sebagai JSON
    let claims: serde_json::Value = match serde_json::from_str(&decrypted) {
        Ok(c) => c,
        Err(_) => {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "code": 401,
                "message": "Token tidak valid!",
                "status": false
            })));
        }
    };

    let user_uuid = claims["user_uuid"].as_str().ok_or("Invalid claims")?;

    // Ambil profil menggunakan service
    match profile_service::my_profile(user_uuid, &pool).await {
        Ok(profile) => Ok(HttpResponse::Ok().json(profile)),
        Err(e) => {
            eprintln!("Error fetching profile: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

fn decrypt_jwe(encrypted_token: &str) -> Result<String, Box<dyn Error>> {
    let key = generate_key();
    let encrypted_bytes = decode(encrypted_token)?;

    let nonce = Nonce::assume_unique_for_key([0u8; 12]); // Ensure this matches the nonce used during encryption
    let tag_len = AES_256_GCM.tag_len();
    let (ciphertext, tag) = encrypted_bytes.split_at(encrypted_bytes.len() - tag_len);

    let mut in_out = ciphertext.to_vec();
    let tag = Tag::try_from(tag);

    let unbound_key = UnboundKey::new(&AES_256_GCM, &key)?;
    let less_safe_key = LessSafeKey::new(unbound_key);

    let _ = less_safe_key.open_in_place(nonce, Aad::empty(), &mut in_out)
        .map_err(|e| {
            eprintln!("Decryption error: {:?}", e);
            Unspecified
        })
        .map(|plaintext| String::from_utf8(plaintext.to_vec()).unwrap_or_else(|_| {
            eprintln!("JWT decoding error: Invalid UTF-8");
            String::new()
        }));

    return Ok(String::from_utf8(in_out).unwrap());
}