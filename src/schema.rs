table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Text,
        password -> Varchar,
        active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
