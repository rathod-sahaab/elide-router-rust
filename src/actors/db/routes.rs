use crate::actix::{Handler, Message};
use crate::diesel::prelude::*;
use crate::models::routes::{NewRoute, Route};
use crate::schema::routes::dsl::{active, routes, slug, target, uuid as auuid};
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
            uuid: Uuid::new_v4(),
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

        // FIXME: set doesn't take optional value
        let activate: bool = match msg.active {
            Some(is_active) => is_active,
            None => true,
        };
        diesel::update(routes)
            .filter(auuid.eq(msg.uuid))
            .set((
                slug.eq(msg.slug),
                target.eq(msg.target),
                active.eq(activate),
            ))
            .get_result::<Route>(&conn)
    }
}

impl Handler<DeleteRoute> for DbActor {
    type Result = QueryResult<Route>;

    fn handle(&mut self, msg: DeleteRoute, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::delete(routes)
            .filter(auuid.eq(msg.uuid))
            .get_result::<Route>(&conn)
    }
}

// impl Handler<PublishRoute> for DbActor {
//     type Result = QueryResult<Route>;

//     fn handle(&mut self, msg: PublishRoute, _: &mut Self::Context) -> Self::Result {
//         let conn = self.0.get().expect("Unable to get a connection");
//         diesel::update(routes)
//             .filter(auuid.eq(msg.uuid))
//             .set(published.eq(true))
//             .get_result::<Route>(&conn)
//     }
// }

// impl Handler<GetRoutes> for DbActor {
//     type Result = QueryResult<Vec<Route>>;

//     fn handle(&mut self, _: GetRoutes, _: &mut Self::Context) -> Self::Result {
//         let conn = self.0.get().expect("Unable to get a connection");
//         routes
//             .filter(published.eq(true))
//             .get_results::<Route>(&conn)
//     }
// }
