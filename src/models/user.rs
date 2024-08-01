use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::{FromRow, Type};

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "gender")]
pub enum Gender {
    #[sqlx(rename = "Laki-laki")]
    LakiLaki,
    #[sqlx(rename = "Perempuan")]
    Perempuan,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Users {
    pub user_uuid: String,
    pub user_application_role_uuid: String,
    pub user_name: String,
    pub user_email: String,
    pub role_title: String,
    pub application_title: String,
    pub division_title: String,
    pub personal_name: String,
    pub personal_birthday: NaiveDate,
    pub personal_gender: Option<Gender>,
    pub personal_phone: String,
    pub personal_address: String,
    pub created_by: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_by: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ApplicationRole {
    pub application_uuid: Uuid,
    pub role_uuid: Uuid,
    pub division_uuid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Register {
    pub user_id: u32,
    pub user_uuid: Uuid,
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
    pub personal_uuid: Uuid,
    pub personal_name: String,
    pub personal_birthday: NaiveDate,
    pub personal_gender: Option<Gender>,
    pub personal_phone: String,
    pub personal_address: String,
    // pub created_by: Option<String>,
    // pub created_at: Option<NaiveDateTime>,
    // pub updated_by: Option<String>,
    // pub updated_at: Option<NaiveDateTime>,
    // pub deleted_by: Option<String>,
    // pub deleted_at: Option<NaiveDateTime>,
    pub application_role: ApplicationRole,
}