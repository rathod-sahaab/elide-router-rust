table! {
    routes (uuid) {
        uuid -> Uuid,
        slug -> Varchar,
        target -> Varchar,
        active -> Bool,
    }
}

table! {
    users (id) {
        id -> Uuid,
        display_name -> Varchar,
        username -> Varchar,
        password_hash -> Varchar,
        email -> Varchar,
        email_verified -> Bool,
        active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    routes,
    users,
);
