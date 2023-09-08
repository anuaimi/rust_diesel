// @generated automatically by Diesel CLI.

diesel::table! {
    bookmarks (id) {
        id -> Int4,
        link -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    trips (id) {
        id -> Int4,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    websites (id) {
        id -> Int4,
        hostname -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bookmarks,
    trips,
    websites,
);
