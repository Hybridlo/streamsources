-- Your SQL goes here

CREATE TABLE twitch_users (
    id bigint PRIMARY KEY,
    username varchar(50) NOT NULL,
    access_token varchar(50) NOT NULL,
    refresh_token varchar(50) NOT NULL,
    creation timestamp without time zone NOT NULL DEFAULT (NOW() at time zone 'utc'),
    last_login timestamp without time zone NOT NULL DEFAULT (NOW() at time zone 'utc'),
    expires_in int NOT NULL,
    scopes text[] NOT NULL,
    broadcaster_type varchar(30) NOT NULL default ''
);

CREATE INDEX twitch_users_username_idx ON twitch_users(username);