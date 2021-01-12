use crate::schema::users;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
/// To get data from DB
pub struct User {
    /// Unique identifier, used by link admin
    pub uuid: Uuid,
    /// Name to be displayed, mostly real name
    pub display_name: String,
    /// Username
    pub username: String,
    /// bcrypt hash of the password
    #[serde(skip_serializing)]
    pub password_hash: String,
    /// email id of user
    pub email: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
/// To insert data in DB
pub struct NewUser {
    /// Unique identifier, used by link admin
    pub uuid: Uuid,
    /// Name to be displayed, mostly real name
    pub display_name: String,
    /// Username
    pub username: String,
    /// bcrypt hash of the password
    pub password_hash: String,
    /// email id of user
    pub email: String,
}
