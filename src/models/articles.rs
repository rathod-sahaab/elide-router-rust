use crate::schema::articles;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Article {
    pub uuid: Uuid,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "articles"]
pub struct NewArticle {
    pub uuid: Uuid,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct ArticleData {
    pub title: String,
    pub body: String,
}
