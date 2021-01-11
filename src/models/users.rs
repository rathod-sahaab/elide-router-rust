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

// FIXME: this is not needed replace when known how to check all optional fields are some
#[derive(Serialize, Deserialize)]
/// To receive data from HTTP request thus Uuid not required
pub struct UserData {
    /// Name to be displayed, mostly real name
    pub display_name: String,
    /// Username
    pub username: String,
    /// bcrypt hash of the password
    pub password: String,
    /// email id of user
    pub email: String,
}

#[derive(Serialize, Deserialize)]
/// To receive data from HTTP request thus Uuid not required
pub struct UpdateUserData {
    /// Name to be displayed, mostly real name
    pub display_name: Option<String>,
    /// Username
    pub username: Option<String>,
    /// bcrypt hash of the password
    pub password: Option<String>,
    /// email id of user
    pub email: Option<String>,
}
