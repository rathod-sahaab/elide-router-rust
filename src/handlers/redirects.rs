use crate::actors::db::routes::ReadRouteBySlug;
use crate::models::AppState;
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};

#[get("/{slug}")]
async fn redirect_by_slug(Path(p_slug): Path<String>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    match db.send(ReadRouteBySlug { slug: p_slug }).await {
        Ok(Ok(route)) => {
            if route.active {
                HttpResponse::TemporaryRedirect()
                    .header("Location", route.target)
                    .finish()
            } else {
                HttpResponse::Found().json("Route inactive")
            }
        }
        Ok(Err(_)) => HttpResponse::NotFound().json("Route not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}
