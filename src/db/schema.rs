table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        update_at -> Timestamp,
    }
}
