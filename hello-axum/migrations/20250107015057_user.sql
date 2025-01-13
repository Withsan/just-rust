-- Add migration script here
drop table user;
create table if not exists user(
    id         BIGINT PRIMARY KEY NOT NULL,
    name       TEXT                NOT NULL
)
