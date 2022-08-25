-- Add migration script here
create table users (
	user_id uuid not null,
	primary key (user_id),
	email text not null,
	username text not null unique,
	password_hash text not null
);
