use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AddApplicationRole{
    pub application_uuid: String,
    pub role_uuid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ApplicationRole {
    pub application_role_uuid: String,
    pub application_title: String,
    pub role_title: String,
    pub created_by: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_by: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ListAppRole{
    pub role_uuid: String,
    pub role_title: String,
}