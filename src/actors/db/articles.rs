use crate::actix::{Handler, Message};
use crate::diesel::prelude::*;
use crate::models::articles::{Article, NewArticle};
use crate::schema::articles::dsl::{articles, body, published, title, uuid as auuid};
use uuid::Uuid;

use crate::actors::db::DbActor;

#[derive(Message)]
#[rtype(result = "QueryResult<Article>")]
pub struct CreateArticle {
    pub title: String,
    pub body: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Article>")]
pub struct UpdateArticle {
    pub uuid: Uuid,
    pub title: String,
    pub body: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Article>")]
pub struct DeleteArticle {
    pub uuid: Uuid,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Article>")]
pub struct PublishArticle {
    pub uuid: Uuid,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Article>>")]
pub struct GetArticles;

impl Handler<CreateArticle> for DbActor {
    type Result = QueryResult<Article>;

    fn handle(&mut self, msg: CreateArticle, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connectio");
        let new_article = NewArticle {
            uuid: Uuid::new_v4(),
            title: msg.title,
            body: msg.body,
        };

        diesel::insert_into(articles)
            .values(new_article)
            .get_result::<Article>(&conn)
    }
}

impl Handler<UpdateArticle> for DbActor {
    type Result = QueryResult<Article>;

    fn handle(&mut self, msg: UpdateArticle, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connectio");

        diesel::update(articles)
            .filter(auuid.eq(msg.uuid))
            .set((title.eq(msg.title), body.eq(msg.body)))
            .get_result::<Article>(&conn)
    }
}

impl Handler<DeleteArticle> for DbActor {
    type Result = QueryResult<Article>;

    fn handle(&mut self, msg: DeleteArticle, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connectio");

        diesel::delete(articles)
            .filter(auuid.eq(msg.uuid))
            .get_result::<Article>(&conn)
    }
}

impl Handler<PublishArticle> for DbActor {
    type Result = QueryResult<Article>;

    fn handle(&mut self, msg: PublishArticle, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connectio");
        diesel::update(articles)
            .filter(auuid.eq(msg.uuid))
            .set(published.eq(true))
            .get_result::<Article>(&conn)
    }
}

impl Handler<GetArticles> for DbActor {
    type Result = QueryResult<Vec<Article>>;

    fn handle(&mut self, _: GetArticles, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connectio");
        articles
            .filter(published.eq(true))
            .get_results::<Article>(&conn)
    }
}
