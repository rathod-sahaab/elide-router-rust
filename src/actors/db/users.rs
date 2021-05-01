use crate::actix::{Handler, Message};
use crate::actors::db::DbActor;
use crate::diesel::prelude::*;
use crate::models::users::{NewUser, User};
use crate::schema::users;
use crate::schema::users::dsl::{email, id, username, users as users_q};

use uuid::Uuid;

// Create messages
#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub username: String,
}

// Read messages
#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct GetUser {
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct GetUserByUsername {
    pub username: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<bool>")]
pub struct CheckUserNameExists {
    pub username: String,
}

// Update messages

// FIXME:
#[derive(Message, AsChangeset)]
#[rtype(result = "QueryResult<User>")]
#[table_name = "users"]
pub struct UpdateUser {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub username: Option<String>,
}

// Delete messages
#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct DeleteUser {
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "bool")]
pub struct UsernameAvailable {
    pub username: String,
}

#[derive(Message)]
#[rtype(result = "bool")]
pub struct EmailAvailable {
    pub email: String,
}

impl Handler<CreateUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        let new_user = NewUser {
            email: msg.email,
            name: msg.name,
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

        users_q.filter(id.eq(msg.id)).get_result::<User>(&conn)
    }
}

impl Handler<GetUserByUsername> for DbActor {
    type Result = QueryResult<User>;
    fn handle(&mut self, msg: GetUserByUsername, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        users_q
            .filter(username.eq(msg.username))
            .get_result::<User>(&conn)
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
            .filter(id.eq(msg.id))
            .set(msg)
            .get_result::<User>(&conn)
    }
}

impl Handler<DeleteUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: DeleteUser, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::delete(users_q)
            .filter(id.eq(msg.id))
            .get_result::<User>(&conn)
    }
}

impl Handler<UsernameAvailable> for DbActor {
    type Result = bool;

    fn handle(&mut self, msg: UsernameAvailable, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        users_q
            .filter(username.eq(msg.username))
            .get_result::<User>(&conn)
            .is_err()
    }
}

impl Handler<EmailAvailable> for DbActor {
    type Result = bool;

    fn handle(&mut self, msg: EmailAvailable, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        users_q
            .filter(email.eq(msg.email))
            .get_result::<User>(&conn)
            .is_err()
    }
}
