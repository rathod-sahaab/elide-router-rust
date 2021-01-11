use crate::actors::db::users::{CreateUser, DeleteUser, UpdateUser};
use crate::models::users::{UpdateUserData, UserData};
use crate::models::AppState;
use crate::utils::crypto::hash;

use actix_web::{
    delete, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use uuid::Uuid;

#[post("/create")]
async fn create_user(user: Json<UserData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();
    let password_hash = hash(user.password).0;
    match db
        .send(CreateUser {
            email: user.email,
            display_name: user.display_name,
            username: user.username,
            // remove null character to store in postgres
            password_hash: password_hash.trim_matches(char::from(0)).to_string(),
        })
        .await
    {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

// TODO: Obtain uuid from JWT
#[put("/{uuid}")]
async fn update_user(
    Path(uuid): Path<Uuid>,
    user: Json<UpdateUserData>,
    state: Data<AppState>,
) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();

    // TODO: Use a more elegant solution
    // let password_hash =
    match db
        .send(UpdateUser {
            uuid,
            email: user.email,
            display_name: user.display_name,
            username: user.username,
            password_hash: user
                .password
                // remove null character to store in postgres
                .map(|pass| hash(pass).0.trim_matches(char::from(0)).to_string()),
        })
        .await
    {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        Ok(Err(_)) => HttpResponse::NotFound().json("User not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

// TODO: Obtain uuid from JWT
#[delete("/{uuid}")]
async fn delete_user(Path(uuid): Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    match db.send(DeleteUser { uuid }).await {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        Ok(Err(_)) => HttpResponse::NotFound().json("User not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}
