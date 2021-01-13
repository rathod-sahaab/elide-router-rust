use crate::actix::{Actor, SyncContext};

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);
impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

pub mod routes;
pub mod users;
