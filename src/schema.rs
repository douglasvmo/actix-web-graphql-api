table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Text,
        password -> Varchar,
        access_type -> Varchar,
        active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
