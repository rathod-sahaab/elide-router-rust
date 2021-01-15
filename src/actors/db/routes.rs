use crate::actix::{Handler, Message};
use crate::diesel::prelude::*;
use crate::models::routes::{NewRoute, Route};
use crate::schema::routes::dsl::*;
use uuid::Uuid;

use crate::actors::db::DbActor;

#[derive(Message)]
#[rtype(result = "QueryResult<Route>")]
pub struct CreateRoute {
    pub slug: String,
    pub target: String,
    pub active: Option<bool>,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Route>")]
pub struct ReadRouteBySlug {
    pub slug: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Route>")]
pub struct UpdateRoute {
    pub uuid: Uuid,
    pub slug: String,
    pub target: String,
    pub active: Option<bool>,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Route>")]
pub struct DeleteRoute {
    pub uuid: Uuid,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Route>")]
pub struct GetRoute {
    pub uuid: Uuid,
}

// TODO: Increment visit and increment unique visit

// #[derive(Message)]
// #[rtype(result = "QueryResult<Route>")]
// pub struct ActivateRoute {
//     pub uuid: Uuid,
// }

// #[derive(Message)]
// #[rtype(result = "QueryResult<Vec<Route>>")]
// pub struct GetRoutes;

impl Handler<CreateRoute> for DbActor {
    type Result = QueryResult<Route>;

    fn handle(&mut self, msg: CreateRoute, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        let new_route = NewRoute {
            slug: msg.slug,
            target: msg.target,
            active: msg.active,
        };

        diesel::insert_into(routes)
            .values(new_route)
            .get_result::<Route>(&conn)
    }
}

impl Handler<ReadRouteBySlug> for DbActor {
    type Result = QueryResult<Route>;
    fn handle(&mut self, msg: ReadRouteBySlug, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        routes.filter(slug.eq(msg.slug)).get_result::<Route>(&conn)
    }
}

impl Handler<UpdateRoute> for DbActor {
    type Result = QueryResult<Route>;

    fn handle(&mut self, msg: UpdateRoute, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::update(routes)
            .filter(id.eq(msg.uuid))
            .set((
                slug.eq(msg.slug),
                target.eq(msg.target),
                active.eq(msg.active.unwrap_or(true)),
            ))
            .get_result::<Route>(&conn)
    }
}

impl Handler<DeleteRoute> for DbActor {
    type Result = QueryResult<Route>;

    fn handle(&mut self, msg: DeleteRoute, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::delete(routes)
            .filter(id.eq(msg.uuid))
            .get_result::<Route>(&conn)
    }
}
