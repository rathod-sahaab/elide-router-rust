use crate::actors::db::users::{CreateUser, DeleteUser, GetUserByUsername, UpdateUser};
use crate::models::AppState;
use crate::utils::crypto::{hash, verify};
use serde::Deserialize;
use validator::Validate;

use actix_web::{
    delete, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use uuid::Uuid;

// FIXME: this is not needed replace when known how to check all optional fields are some
#[derive(Deserialize, Validate)]
/// To receive data from HTTP request thus Uuid not required
pub struct UserData {
    /// Name to be displayed, mostly real name
    pub display_name: String,
    /// Username
    #[validate(length(min = 3))]
    pub username: String,
    /// password string
    #[validate(length(min = 6))]
    pub password: String,
    /// email id of user
    #[validate(email)]
    pub email: String,
}

#[derive(Deserialize, Validate)]
/// To receive data from HTTP request thus Uuid not required
pub struct UpdateUserData {
    /// Name to be displayed, mostly real name
    pub display_name: Option<String>,
    /// Username
    #[validate(length(min = 3))]
    pub username: Option<String>,
    /// password
    #[validate(length(min = 6))]
    pub password: Option<String>,
    /// email id of user
    #[validate(email)]
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

    if let Err(errors) = user.validate() {
        let error_map = errors.field_errors();

        let message = if error_map.contains_key("username") {
            format!("Invalid username. \"{}\" is too short.", user.username)
        } else if error_map.contains_key("email") {
            format!("Invalid email address \"{}\"", user.email)
        } else if error_map.contains_key("password") {
            "Invalid password. Too short".to_string()
        } else {
            "Invalid input.".to_string()
        };

        return HttpResponse::BadRequest().json(message);
    }

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

// TODO: Obtain id from JWT
#[put("/{id}")]
async fn update_user(
    Path(id): Path<Uuid>,
    user: Json<UpdateUserData>,
    state: Data<AppState>,
) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();

    if let Err(errors) = user.validate() {
        let error_map = errors.field_errors();

        let message = if error_map.contains_key("username") {
            format!(
                "Invalid username. \"{}\" is too short.",
                user.username.unwrap()
            )
        } else if error_map.contains_key("email") {
            format!("Invalid email address \"{}\"", user.email.unwrap())
        } else if error_map.contains_key("password") {
            "Invalid password. Too short".to_string()
        } else {
            "Invalid input.".to_string()
        };

        return HttpResponse::BadRequest().json(message);
    }

    match db
        .send(UpdateUser {
            id,
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

// TODO: Obtain id from JWT
#[delete("/{id}")]
async fn delete_user(Path(id): Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    match db.send(DeleteUser { id }).await {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        Ok(Err(_)) => HttpResponse::NotFound().json("User not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}
