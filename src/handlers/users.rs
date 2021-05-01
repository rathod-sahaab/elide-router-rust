use crate::actors::db::users::{CreateUser, DeleteUser, GetUser, GetUserByUsername, UpdateUser};
use crate::models::AppState;
use crate::utils::crypto::{hash, verify};
use actix_session::Session;
use serde::{Deserialize, Serialize};
use validator::Validate;

use actix_web::{
    delete, get, post, put,
    web::{Data, Json},
    HttpResponse, Responder,
};
use uuid::Uuid;

// FIXME: this is not needed replace when known how to check all optional fields are some
#[derive(Serialize, Deserialize, Validate, Debug)]
/// To receive data from HTTP request thus Uuid not required
pub struct UserData {
    /// Name to be displayed, mostly real name
    pub name: String,
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
    pub name: Option<String>,
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

#[derive(Debug, Serialize, Deserialize)]
struct UserLoginData {
    username: String,
    password: String,
}

#[post("/register")]
async fn register_user(user: Json<UserData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();
    info!("User is registering");
    info!("{:?}", user);

    if let Err(errors) = user.validate() {
        let error_map = errors.field_errors();

        let message = if error_map.contains_key("username") {
            format!("Invalid username. '{}' is too short.", user.username)
        } else if error_map.contains_key("email") {
            format!("Invalid email address '{}'", user.email)
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
            name: user.name,
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

#[get("/me")]
async fn me_user(session: Session, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user_id: Option<Uuid> = session.get("user_id").unwrap_or(None);

    if user_id.is_none() {
        return HttpResponse::Unauthorized().json("Unauthorized");
    }
    let user_id = user_id.unwrap();

    match db.send(GetUser { id: user_id }).await {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        Ok(Err(_)) => HttpResponse::NotFound().json("User not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

// fn user_id_from_session(session: Session) -> Result<Uuid, actix_web::HttpResponse> {
//     let user_id: Option<Uuid> = session.get("user_id").unwrap_or(None);

//     if user_id.is_none() {
//         return Err(HttpResponse::Unauthorized().json("Unauthorized"));
//     }
//     Ok(user_id.unwrap())
// }

#[post("/login")]
async fn login_user(
    login_data: Json<UserLoginData>,
    session: Session,
    state: Data<AppState>,
) -> impl Responder {
    info!("{:?}", login_data);
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
                session.set("user_id", user.id).unwrap();
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

#[get("/loggedin")]
async fn loggedin_check(session: Session) -> impl Responder {
    let user_id: Option<Uuid> = session.get("user_id").unwrap_or(None);
    if user_id.is_none() {
        return HttpResponse::Unauthorized().json("Unauthorized");
    }
    HttpResponse::Ok().json(true)
}

#[get("/logout")]
async fn logout_user(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Ok().json(true)
}

#[put("/update")]
async fn update_user(
    user: Json<UpdateUserData>,
    session: Session,
    state: Data<AppState>,
) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user = user.into_inner();
    let user_id: Option<Uuid> = session.get("user_id").unwrap_or(None);

    if user_id.is_none() {
        return HttpResponse::Unauthorized().json("Unauthorized");
    }
    let user_id = user_id.unwrap();

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
            id: user_id,
            email: user.email,
            name: user.name,
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

#[delete("/delete")]
async fn delete_user(session: Session, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user_id: Option<Uuid> = session.get("user_id").unwrap_or(None);

    if user_id.is_none() {
        return HttpResponse::Unauthorized().json("Unauthorized");
    }
    let user_id = user_id.unwrap();

    match db.send(DeleteUser { id: user_id }).await {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        Ok(Err(_)) => HttpResponse::NotFound().json("User not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}
