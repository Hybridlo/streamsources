// @generated automatically by Diesel CLI.

diesel::table! {
    twitch_users (id) {
        id -> Int8,
        username -> Varchar,
        access_token -> Varchar,
        refresh_token -> Varchar,
        creation -> Timestamp,
        last_login -> Timestamp,
        expires_in -> Int4,
        scopes -> Array<Nullable<Text>>,
        broadcaster_type -> Varchar,
    }
}
