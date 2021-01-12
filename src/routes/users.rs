use crate::actors::db::users::{CreateUser, DeleteUser, GetUserByUsername, UpdateUser};
use crate::models::AppState;
use crate::utils::crypto::{hash, verify};
use serde::{Deserialize, Serialize};

use actix_web::{
    delete, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use uuid::Uuid;

// FIXME: this is not needed replace when known how to check all optional fields are some
#[derive(Serialize, Deserialize)]
/// To receive data from HTTP request thus Uuid not required
pub struct UserData {
    /// Name to be displayed, mostly real name
    pub display_name: String,
    /// Username
    pub username: String,
    /// password string
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
#[derive(Deserialize)]
struct UserLoginData {
    username: String,
    password: String,
}

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

#[post("/login")]
async fn login_user(login_data: Json<UserLoginData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let login_data = login_data.into_inner();
    match db
        .send(GetUserByUsername {
            username: login_data.username,
        })
        .await
    {
        Ok(Ok(user)) => {
            if verify(&user.password_hash, login_data.password) {
                HttpResponse::Ok().json(user)
            } else {
                // prevent brute forcing usernames
                HttpResponse::NotFound().json("Username or password wrong")
            }
        }
        // prevent brute forcing usernames
        Ok(Err(_err)) => HttpResponse::NotFound().json("Username or password wrong"),
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
