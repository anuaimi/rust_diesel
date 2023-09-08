// @generated automatically by Diesel CLI.

diesel::table! {
    bookmarks (id) {
        id -> Int4,
        link -> Varchar,
        trip_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    trips (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    websites (id) {
        id -> Int4,
        hostname -> Varchar,
        scraped -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(bookmarks -> trips (trip_id));

diesel::allow_tables_to_appear_in_same_query!(
    bookmarks,
    trips,
    websites,
);
