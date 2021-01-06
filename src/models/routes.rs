use crate::schema::routes;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
/// To get data from DB
pub struct Route {
    /// Unique identifier, used by link admin
    pub uuid: Uuid,
    /// slug part of elide URL, elide.com/this-is-slug
    pub slug: String,
    /// Target where requestee should be redirected
    pub target: String,
    /// Is the link active
    pub active: bool,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "routes"]
/// To insert data in DB
pub struct NewRoute {
    /// Unique identifier, used by link admin
    pub uuid: Uuid,
    /// slug part of elide URL, elide.com/this-is-slug
    pub slug: String,
    /// Target where requestee should be redirected
    pub target: String,
    /// Is the link active
    pub active: Option<bool>,
}

#[derive(Serialize, Deserialize)]
/// To receive data from HTTP request thus Uuid not necessary
pub struct RouteData {
    /// slug part of elide URL, elide.com/this-is-slug
    pub slug: String,
    /// Target where requestee should be redirected
    pub target: String,
    /// Is the link active
    pub active: Option<bool>,
}
