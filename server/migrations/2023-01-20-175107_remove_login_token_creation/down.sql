-- This file should undo anything in `up.sql`
alter table quick_login_token add creation timestamp without time zone NOT NULL DEFAULT (NOW() at time zone 'utc');
update quick_login_token
set creation = to_timestamp(0);