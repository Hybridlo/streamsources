-- Your SQL goes here
create table subscription (
    id bigserial primary KEY,
    user_id bigint,
    secret varchar(100) not null,
    sub_id varchar(100) not null,
    type varchar(100) not null,
    last_connect timestamp without time zone NOT NULL DEFAULT (NOW() at time zone 'utc'),
    last_disconnect timestamp without time zone NOT NULL DEFAULT (NOW() at time zone 'utc'),

    constraint unique_sub_id unique (sub_id),
    constraint fk_user
        foreign key (user_id)
            references twitch_users(id)
            on delete cascade
);

create index idx_subscription_sub_id
on subscription(sub_id);