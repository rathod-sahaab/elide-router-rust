use crate::actors::db::routes::{CreateRoute, DeleteRoute, UpdateRoute};
use crate::models::routes::RouteData;
use crate::models::AppState;
use actix_session::Session;
use diesel::result::Error::DatabaseError;
use serde::Deserialize;

use actix_web::{
    delete, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use uuid::Uuid;

#[post("/create")]
async fn create_route(
    route: Json<RouteData>,
    session: Session,
    state: Data<AppState>,
) -> impl Responder {
    let db = state.as_ref().db.clone();
    let route = route.into_inner();

    let user_id: Option<Uuid> = session.get("user_id").unwrap_or(None);

    if user_id.is_none() {
        return HttpResponse::Unauthorized().json("Unauthorized");
    }

    match db
        .send(CreateRoute {
            slug: route.slug,
            creator_id: user_id,
            target: route.target,
            active: route.active,
        })
        .await
    {
        Ok(Ok(route)) => HttpResponse::Ok().json(route),
        Ok(Err(error)) => match error {
            DatabaseError(_, _) => {
                HttpResponse::BadRequest().json("Route with this slug already exists")
            },
            _ => HttpResponse::BadRequest().json("Request was bad"),
        },
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

/// Create orphan route is only for demo purposes this lacks creator and is purged at UTC midnight
#[post("/create-orphan")]
async fn create_orphan_route(
    route: Json<RouteData>,
    session: Session,
    state: Data<AppState>,
) -> impl Responder {
    let db = state.as_ref().db.clone();
    let route = route.into_inner();

    if session.get::<Uuid>("user_id").is_ok() {
        // User has a valid session and hence has an account so they should use /create
        return HttpResponse::BadRequest().json(
            "You are already a user, app should use /api/routes/create and not /create-orphan",
        );
    }
    match db
        .send(CreateRoute {
            slug: route.slug,
            creator_id: None,
            target: route.target,
            active: route.active,
        })
        .await
    {
        Ok(Ok(route)) => HttpResponse::Ok().json(route),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[derive(Deserialize)]
pub struct UpdateRouteData {
    /// ID of route to be updated
    id: Uuid,
    /// slug part of elide URL, elide.com/this-is-slug
    pub slug: String,
    /// Target where requestee should be redirected
    pub target: String,
    /// Is the link active
    pub active: Option<bool>,
}

#[put("/update")]
async fn update_route(
    session: Session,
    route: Json<UpdateRouteData>,
    state: Data<AppState>,
) -> impl Responder {
    let db = state.as_ref().db.clone();
    let route = route.into_inner();

    let user_id: Option<Uuid> = session.get("user_id").unwrap_or(None);
    if user_id.is_none() {
        return HttpResponse::Unauthorized().json("Unauthorized");
    }
    let user_id: Uuid = user_id.unwrap();

    match db
        .send(UpdateRoute {
            id: route.id,
            creator_id: user_id,
            slug: route.slug,
            target: route.target,
            active: route.active,
        })
        .await
    {
        Ok(Ok(route)) => HttpResponse::Ok().json(route),
        Ok(Err(_)) => HttpResponse::NotFound().json("Route not found, or unauthorized access"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[delete("/{uuid}")]
async fn delete_route(Path(uuid): Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    match db.send(DeleteRoute { uuid }).await {
        Ok(Ok(route)) => HttpResponse::Ok().json(route),
        Ok(Err(_)) => HttpResponse::NotFound().json("Route not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}
