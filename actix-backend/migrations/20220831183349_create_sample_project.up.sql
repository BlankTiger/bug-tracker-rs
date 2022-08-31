-- Add up migration script here
insert into projects (project_name, desc_short, desc_long, status, assigned_to, created_by) 
values ('Sample project', 'Sample project description', 'Sample project description', 'active', array [(select user_id from users where username='admin')], (select user_id from users where username='admin'));
