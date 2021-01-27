table! {
    routes (id) {
        id -> Uuid,
        slug -> Varchar,
        creator_id -> Nullable<Uuid>,
        target -> Varchar,
        active -> Bool,
        active_from -> Nullable<Timestamp>,
        active_till -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        username -> Varchar,
        password_hash -> Varchar,
        email -> Varchar,
        email_verified -> Bool,
        active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(routes -> users (creator_id));

allow_tables_to_appear_in_same_query!(
    routes,
    users,
);
