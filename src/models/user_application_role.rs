use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::{FromRow, Type};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserApplicationRole {
    pub application_role_id: i32,
    pub application_role_uuid: Uuid,
    pub application_id: i32,
    pub role_id: i32,
    pub application_title: Option<String>,
    pub role_title: Option<String>,
    pub created_by: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_by: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
}