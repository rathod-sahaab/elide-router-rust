table! {
    articles (uuid) {
        uuid -> Uuid,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    routes (uuid) {
        uuid -> Uuid,
        slug -> Varchar,
        target -> Varchar,
        active -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    articles,
    routes,
);
