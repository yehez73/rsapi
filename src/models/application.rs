use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::{FromRow, Type};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Application {
    // pub application_uuid: Uuid,
    // pub application_order: i32,
    pub application_code: String,
    pub application_title: String,
    pub application_description: Option<String>,
    // pub application_show: bool,
    pub created_by: Option<String>,
    // pub created_at: Option<NaiveDateTime>,
    // pub updated_by: Option<String>,
    // pub updated_at: Option<NaiveDateTime>,
    // pub deleted_by: Option<String>,
    // pub deleted_at: Option<NaiveDateTime>,
}