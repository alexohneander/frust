// @generated automatically by Diesel CLI.

diesel::table! {
    feeds (id) {
        id -> Int4,
        title -> Varchar,
        url -> Varchar,
        feedtype -> Varchar,
    }
}
