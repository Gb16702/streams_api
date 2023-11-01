create table utilisateur (
    id serial primary key,
    email varchar not null,
    password varchar not null,
    is_admin boolean not null default false
);