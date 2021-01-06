extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod actors;
mod db_utils;
mod models;
mod routes;
mod schema;

use actix_web::{web::scope, App, HttpServer};

use actix::SyncArbiter;
use actors::db::DbActor;
use db_utils::{get_pool, run_migrations};
use models::AppState;
use std::env;

use routes::{
    articles::{create_article, delete_article, get_published, publish_article, update_article},
    redirects::redirect_by_slug,
    routes::{create_route, delete_route, update_route},
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
                        scope("/articles/")
                            .service(get_published)
                            .service(delete_article)
                            .service(publish_article)
                            .service(create_article)
                            .service(update_article),
                    )
                    .service(
                        scope("/routes/")
                            .service(create_route)
                            .service(update_route)
                            .service(delete_route),
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
