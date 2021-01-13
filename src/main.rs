extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

extern crate chrono;
extern crate sodiumoxide;

mod actors;
mod handlers;
mod models;
mod schema;
mod utils;

use actix_web::{web::scope, App, HttpServer};

use actix::SyncArbiter;
use actors::db::DbActor;
use models::AppState;
use std::env;
use utils::db::{get_pool, run_migrations};

use handlers::{
    redirects::redirect_by_slug,
    routes::{create_route, delete_route, update_route},
    users::{create_user, delete_user, login_user, update_user},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = env::var("DATABASE_URL").expect("Error retrieving the database url");
    run_migrations(&db_url);
    let pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .service(
                scope("/api/")
                    .service(
                        scope("/routes/")
                            .service(create_route)
                            .service(update_route)
                            .service(delete_route),
                    )
                    .service(
                        scope("/users/")
                            .service(create_user)
                            .service(login_user)
                            .service(update_user)
                            .service(delete_user),
                    ),
            )
            .service(redirect_by_slug)
            .data(AppState {
                db: db_addr.clone(),
            })
    })
    .bind(("0.0.0.0", 9600))?
    .run()
    .await
}
