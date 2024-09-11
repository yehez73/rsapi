use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GetRole {
    pub role_uuid: String,
    pub role_order: i32,
    pub role_code: String,
    pub role_title: String,
    pub role_show: bool,
    pub created_by: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_by: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub role_code: String,
    pub role_title: String,
    pub created_by: Option<String>,
}