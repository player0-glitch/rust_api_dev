// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Nullable<Integer>,
        artist -> Text,
        release_year -> Integer,
        sales -> Integer,
        released -> Bool,
    }
}
