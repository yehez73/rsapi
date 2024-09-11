use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "gender")]
pub enum Gender {
    #[serde(rename = "Laki-laki")]
    #[sqlx(rename = "Laki-laki")]
    LakiLaki,
    #[serde(rename = "Perempuan")]
    #[sqlx(rename = "Perempuan")]
    Perempuan,
}

impl ToString for Gender {
    fn to_string(&self) -> String {
        match self {
            Gender::LakiLaki => "Laki-laki".to_string(),
            Gender::Perempuan => "Perempuan".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserApplicationRole {
    pub user_uuid: String,
    pub user_application_role_uuid: String,
    pub user_name: String,
    pub user_email: String,
    pub role_title: String,
    pub application_title: String,
    pub division_title: String,
    pub personal_name: String,
    pub personal_birthday: Option<NaiveDate>,
    pub personal_gender: Gender,
    pub personal_phone: String,
    pub personal_address: String,
    pub created_by: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}