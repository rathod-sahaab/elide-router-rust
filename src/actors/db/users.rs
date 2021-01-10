use crate::actix::{Handler, Message};
use crate::diesel::prelude::*;
use crate::models::users::{NewUser, User};
use crate::schema::users;
use crate::schema::users::dsl::{username, users as users_q, uuid as auuid};
use uuid::Uuid;

use crate::actors::db::DbActor;

// Create messages
#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct CreateUser {
    pub display_name: String,
    pub email: String,
    pub password_hash: String,
    pub username: String,
}

// Read messages
#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct GetUser {
    pub uuid: Uuid,
}

#[derive(Message)]
#[rtype(result = "QueryResult<bool>")]
pub struct CheckUserNameExists {
    pub username: String,
}

// Update messages

// #[derive(Message)]
// #[rtype(result = "QueryResult<bool>")]
// pub struct UpdateUsername {
//     pub uuid: Uuid,
//     pub username: String,
// }

// #[derive(Message)]
// #[rtype(result = "QueryResult<bool>")]
// pub struct UpdateEmail {
//     pub uuid: Uuid,
//     pub email: String,
// }

// #[derive(Message)]
// #[rtype(result = "QueryResult<bool>")]
// pub struct UpdateDisplayName {
//     pub uuid: Uuid,
//     pub display_name: String,
// }

// #[derive(Message)]
// #[rtype(result = "QueryResult<bool>")]
// pub struct UpdatePassword {
//     pub uuid: Uuid,
//     pub password_hash: String,
// }

// FIXME:
#[derive(Message, AsChangeset)]
#[rtype(result = "QueryResult<User>")]
#[table_name = "users"]
pub struct UpdateUser {
    pub uuid: Uuid,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub username: Option<String>,
}

// Delete messages
#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct DeleteUser {
    pub uuid: Uuid,
}

impl Handler<CreateUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        let new_user = NewUser {
            uuid: Uuid::new_v4(),
            email: msg.email,
            display_name: msg.display_name,
            username: msg.username,
            password_hash: msg.password_hash,
        };

        diesel::insert_into(users_q)
            .values(new_user)
            .get_result::<User>(&conn)
    }
}

impl Handler<GetUser> for DbActor {
    type Result = QueryResult<User>;
    fn handle(&mut self, msg: GetUser, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        users_q.filter(auuid.eq(msg.uuid)).get_result::<User>(&conn)
    }
}
impl Handler<CheckUserNameExists> for DbActor {
    type Result = QueryResult<bool>;
    fn handle(&mut self, msg: CheckUserNameExists, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        Ok(users_q
            .filter(username.eq(msg.username))
            .get_result::<User>(&conn)
            .is_ok())
    }
}

impl Handler<UpdateUser> for DbActor {
    type Result = QueryResult<User>;
    // FIXME: As change set
    fn handle(&mut self, msg: UpdateUser, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::update(users_q)
            .filter(auuid.eq(msg.uuid))
            .set(msg)
            .get_result::<User>(&conn)
    }
}

impl Handler<DeleteUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: DeleteUser, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::delete(users_q)
            .filter(auuid.eq(msg.uuid))
            .get_result::<User>(&conn)
    }
}
