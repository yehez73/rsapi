use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Users {
    pub user_uuid: String,
    pub user_application_role_uuid: String,
    pub user_name: String,
    pub user_email: String,
    pub role_title: String,
    pub application_title: String,
    pub division_title: String,
    pub personal_name: String,
    pub personal_birthday: String,
    pub personal_gender: String,
    pub personal_phone: String,
    pub personal_address: String,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_by: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
}