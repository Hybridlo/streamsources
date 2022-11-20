-- Your SQL goes here
CREATE TABLE quick_login_token (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    token varchar(30) NOT NULL,
    creation timestamp without time zone NOT NULL DEFAULT (NOW() at time zone 'utc'),

    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
            REFERENCES twitch_users(id)
            ON DELETE CASCADE
);