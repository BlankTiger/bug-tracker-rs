-- Add up migration script here
create table projects (
	project_id serial primary key,
	project_name text not null,
	desc_short text not null,
	desc_long text,
	status text not null,
	assigned_to uuid[] not null,
	created_by uuid not null references users(user_id),
	created_at timestamp not null default now()
);
