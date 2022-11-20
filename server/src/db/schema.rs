// @generated automatically by Diesel CLI.

diesel::table! {
    auth_state (id) {
        id -> Int8,
        state -> Varchar,
        creation -> Timestamp,
    }
}

diesel::table! {
    quick_login_token (id) {
        id -> Int8,
        user_id -> Int8,
        token -> Varchar,
        creation -> Timestamp,
    }
}

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

diesel::joinable!(quick_login_token -> twitch_users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_state,
    quick_login_token,
    twitch_users,
);
