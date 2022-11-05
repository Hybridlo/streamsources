-- Your SQL goes here
CREATE TABLE auth_state (
    id BIGSERIAL PRIMARY KEY,
    state varchar(50) NOT NULL,
    creation timestamp without time zone NOT NULL DEFAULT (NOW() at time zone 'utc')
);

CREATE INDEX auth_state_state_idx ON auth_state(state);