use crate::actix::Addr;
use crate::actors::db::DbActor;

pub struct AppState {
    pub db: Addr<DbActor>,
}

pub mod routes;
pub mod users;
