table! {
    pastes (id) {
        id -> Int4,
        slug -> Varchar,
        delete_token -> Varchar,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}
