// @generated automatically by Diesel CLI.

diesel::table! {
    auth_state (id) {
        id -> Int8,
        #[max_length = 50]
        state -> Varchar,
        creation -> Timestamp,
    }
}

diesel::table! {
    quick_login_token (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 30]
        token -> Varchar,
    }
}

diesel::table! {
    subscription (id) {
        id -> Int8,
        user_id -> Nullable<Int8>,
        #[max_length = 100]
        secret -> Varchar,
        #[max_length = 100]
        sub_id -> Varchar,
        #[sql_name = "type"]
        #[max_length = 100]
        type_ -> Varchar,
        connected -> Bool,
        inactive_since -> Timestamp,
    }
}

diesel::table! {
    twitch_users (id) {
        id -> Int8,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 50]
        access_token -> Varchar,
        #[max_length = 50]
        refresh_token -> Varchar,
        creation -> Timestamp,
        last_login -> Timestamp,
        expires_in -> Int4,
        scopes -> Array<Nullable<Text>>,
        #[max_length = 30]
        broadcaster_type -> Varchar,
    }
}

diesel::joinable!(quick_login_token -> twitch_users (user_id));
diesel::joinable!(subscription -> twitch_users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_state,
    quick_login_token,
    subscription,
    twitch_users,
);
