use crate::actors::db::routes::{CreateRoute, DeleteRoute, UpdateRoute};
use crate::models::routes::RouteData;
use crate::models::AppState;

use actix_web::{
    delete, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use uuid::Uuid;

#[post("/create")]
async fn create_route(route: Json<RouteData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let route = route.into_inner();

    match db
        .send(CreateRoute {
            slug: route.slug,
            target: route.target,
            active: route.active,
        })
        .await
    {
        Ok(Ok(route)) => HttpResponse::Ok().json(route),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[put("/{uuid}")]
async fn update_route(
    Path(uuid): Path<Uuid>,
    route: Json<RouteData>,
    state: Data<AppState>,
) -> impl Responder {
    let db = state.as_ref().db.clone();
    let route = route.into_inner();

    match db
        .send(UpdateRoute {
            uuid,
            slug: route.slug,
            target: route.target,
            active: route.active,
        })
        .await
    {
        Ok(Ok(route)) => HttpResponse::Ok().json(route),
        Ok(Err(_)) => HttpResponse::NotFound().json("Route not found"),
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
