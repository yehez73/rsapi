use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::{FromRow, Type};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Division {
    // pub division_uuid: Uuid,
    // pub division_order: i32,
    pub division_code: String,
    pub division_title: String,
    // pub division_show: bool,
    pub created_by: Option<String>,
    // pub created_at: Option<NaiveDateTime>,
    // pub updated_by: Option<String>,
    // pub updated_at: Option<NaiveDateTime>,
    // pub deleted_by: Option<String>,
    // pub deleted_at: Option<NaiveDateTime>,
}