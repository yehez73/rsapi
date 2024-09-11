use anyhow::{Result, Context as _};
use base64::{engine::general_purpose, Engine as _};
use rand::{rngs::OsRng, RngCore as _};
use dotenv::dotenv;
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use std::env;

pub fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);

    dotenv().ok();
    env::set_var("ENCRYPTION_KEY", hex::encode(key));
    key
}

pub fn get_key() -> [u8; 32] {
    dotenv().ok();
    match env::var("ENCRYPTION_KEY") {
        Ok(key_str) => {
            let key = hex::decode(key_str).unwrap();
            key.try_into().unwrap()
        }
        Err(_) => {
            let key = generate_key();
            key
        }
    }
}

pub fn encrypt_token(token: &str, key: [u8; 32]) -> Result<String> {
    let mut nonce_bytes = [0u8; 12];
    OsRng
        .try_fill_bytes(&mut nonce_bytes)
        .map_err(|_| anyhow::anyhow!("Failed to generate nonce"))?;
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);

    let less_safe_key = LessSafeKey::new(
        UnboundKey::new(&AES_256_GCM, &key).context("Failed to create encryption key")?
    );

    let mut token_bytes = token.as_bytes().to_vec();
    less_safe_key
        .seal_in_place_append_tag(nonce, Aad::empty(), &mut token_bytes)
        .context("Encryption failed")?;

    let mut encrypted_data = nonce_bytes.to_vec();
    encrypted_data.extend_from_slice(&token_bytes);

    Ok(general_purpose::STANDARD.encode(&encrypted_data))
}

pub fn decrypt_token(encrypted_token: &str, key: &[u8]) -> Result<String> {
    let encrypted_data = general_purpose::STANDARD
        .decode(encrypted_token)
        .context("Failed to decode base64 token")?;

    if encrypted_data.len() < 12 {
        anyhow::bail!("Invalid token length: less than 12 bytes.");
    }

    let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
    let nonce = Nonce::assume_unique_for_key(nonce_bytes.try_into().unwrap());

    let less_safe_key = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, key)?);

    let mut decrypted_data = ciphertext.to_vec();

    let plaintext = less_safe_key
        .open_in_place(nonce, Aad::empty(), &mut decrypted_data)
        .context("Decryption error")?;
    
    String::from_utf8(plaintext.to_vec()).context("Failed to convert decrypted data to UTF-8")
}