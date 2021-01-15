use crate::schema::routes;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use chrono::NaiveDateTime;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
/// To get data from DB
pub struct Route {
    /// Unique identifier, used by link admin
    pub id: Uuid,
    /// slug part of elide URL, elide.com/this-is-slug
    pub slug: String,
    /// Target where requestee should be redirected
    pub target: String,
    pub creator_id: Option<Uuid>,
    /// Is the link active
    pub active: bool,
    // Time from which the link will be active
    pub active_from: Option<NaiveDateTime>,
    // Time till which the link should be active
    pub active_till: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "routes"]
/// To insert data in DB
pub struct NewRoute {
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
