-- Add migration script here
insert into users (user_id, email, username, password_hash)
values ( 
    'ddf8994f-d522-4659-8d02-c1d479057be6',
	'maciej-urban@outlook.com',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$OEx/rcq+3ts//WUDzGNl2g$Am8UFBA4w5NJEmAtquGvBmAlu92q/VQcaoL5AyJPfc8'
);
