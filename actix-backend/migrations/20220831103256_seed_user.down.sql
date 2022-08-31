-- Add down migration script here
delete from users where username = 'admin';
