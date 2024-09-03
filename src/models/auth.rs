use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::{FromRow, Type};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Login {
    pub user_email: String,
    pub user_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDetails {
    pub user_uuid: String,
    pub role_code: String,
    pub division_title: String,
    pub division_code: String,
    pub user_name: String,
    pub user_id: i64,
    pub is_authenticated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePassword {
    pub old_password: String,
    pub new_password: String,
}