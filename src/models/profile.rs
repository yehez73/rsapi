use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Profile {
    pub user_uuid: String,
    pub user_name: String,
    pub role_code: String,
}