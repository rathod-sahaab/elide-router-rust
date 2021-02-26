extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

extern crate chrono;
extern crate sodiumoxide;
extern crate validator;

mod actors;
mod handlers;
mod models;
mod schema;
mod utils;

use actix_web::{cookie, http, web::scope, App, HttpServer};

use actix::SyncArbiter;
use actix_cors::Cors;
use actix_redis::RedisSession;
use std::env;

use actors::db::DbActor;
use models::AppState;
use utils::{
    crypto::random_redis_key,
    db::{get_pool, run_migrations},
};

use handlers::{
    redirects::{redirect_by_slug, redirect_to_console},
    routes::{create_route, delete_route, get_user_routes, update_route},
    users::{delete_user, login_user, logout_user, me_user, register_user, update_user},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = env::var("DATABASE_URL").expect("Error retrieving the database url");
    run_migrations(&db_url);
    let pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));
    let redis_key = random_redis_key(); // fetch redis key here so all threads have same key

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                // FIXME: only run in development
                if cfg!(debug_assertions) {
                    origin.as_bytes().starts_with(b"http://localhost")
                } else {
                    false
                }
            })
            // TODO: pick from config
            .allowed_origin_fn(|origin, _req_head| {
                // FIXME: only run in development
                origin.as_bytes().ends_with(b".elide.me") // all sub domains
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            // cookie session middleware
            .wrap(
                // TODO: no magic, just config
                RedisSession::new("redis:6379", &redis_key)
                    // don't allow the cookie to be accessed from javascript
                    .cookie_http_only(true)
                    .cookie_same_site(cookie::SameSite::Lax),
            )
            .service(
                scope("/api/")
                    .service(
                        scope("/routes/")
                            .service(create_route)
                            .service(get_user_routes)
                            .service(update_route)
                            .service(delete_route),
                    )
                    .service(
                        scope("/users/")
                            .service(register_user)
                            .service(me_user)
                            .service(login_user)
                            .service(logout_user)
                            .service(update_user)
                            .service(delete_user),
                    ),
            )
            .service(redirect_by_slug)
            .service(redirect_to_console)
            .data(AppState {
                db: db_addr.clone(),
            })
    })
    .bind(("0.0.0.0", 9600))?
    .run()
    .await
}
