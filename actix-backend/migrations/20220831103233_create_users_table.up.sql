-- Add up migration script here
create table users (
	user_id uuid primary key,
	username text not null unique,
	password_hash text not null,
	email text not null,
	role text not null
);
