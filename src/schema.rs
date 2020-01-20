table! {
    user (id) {
        id -> Bigint,
        username -> Varchar,
        password_hash -> Varchar,
        salt -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
