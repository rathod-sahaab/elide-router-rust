use crate::actors::db::routes::RouteSlugAvailable;
use crate::actors::db::users::{EmailAvailable, UsernameAvailable};
use crate::models::AppState;
use serde::{Deserialize, Serialize};

use actix_web::{
    get,
    web::{Data, Json},
    HttpResponse, Responder,
};

#[derive(Serialize, Debug)]
struct Availability {
    available: bool,
}

#[derive(Deserialize, Debug)]
struct Username {
    pub username: String,
}

#[derive(Deserialize, Debug)]
struct Email {
    pub email: String,
}

#[derive(Deserialize, Debug)]
struct Slug {
    pub slug: String,
}

#[get("/username")]
async fn username_availability(username: Json<Username>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let username = username.into_inner();
    let result = db
        .send(UsernameAvailable {
            username: username.username,
        })
        .await;
    return HttpResponse::Ok().json(Availability {
        available: result.unwrap(),
    });
}

#[get("/email")]
async fn email_availability(email: Json<Email>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let email = email.into_inner();
    let result = db.send(EmailAvailable { email: email.email }).await;
    return HttpResponse::Ok().json(Availability {
        available: result.unwrap(),
    });
}

#[get("/slug")]
async fn slug_availability(slug: Json<Slug>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let slug = slug.into_inner();
    let result = db.send(RouteSlugAvailable { slug: slug.slug }).await;
    return HttpResponse::Ok().json(Availability {
        available: result.unwrap(),
    });
}
