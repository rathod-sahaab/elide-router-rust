use crate::schema::users;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use chrono::NaiveDateTime;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
/// To get data from DB
pub struct User {
    /// Unique identifier, used by link admin
    pub id: Uuid,
    /// Name to be displayed, mostly real name
    pub display_name: String,
    pub username: String,
    /// bcrypt hash of the password
    #[serde(skip_serializing)]
    pub password_hash: String,
    /// email id of user
    pub email: String,
    #[serde(skip_serializing)]
    pub email_verified: bool,
    #[serde(skip_serializing)]
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
/// To insert data in DB
pub struct NewUser {
    /// Name to be displayed, mostly real name
    pub display_name: String,
    /// Username
    pub username: String,
    /// bcrypt hash of the password
    pub password_hash: String,
    /// email id of user
    pub email: String,
}
