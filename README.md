# bug-tracker-rs
Fullstack web app written completely in rust. Yew for the frontend, actix for the backend. Utilises ssl, postgres, and redis. Implements session based authentication.

# Using make to interact with the program
You have to have dotenv-rust installed, then using a posix compliant shell, you can do:
```bash
# run the frontend
make serve-frontend

# copy over the css files on change
make update-css

# run the backend
make serve-backend
```

# Goals
- [ ] Yew frontend
	- [x] create a sidebar
		- [x] make the sidebar collapsible
	- [ ] create a dashboard
		- [ ] create a pie chart of types and numbers of bugs
		- [ ] create a table of issues
		- [ ] create a table of projects
	- [ ] create a projects page
	- [ ] create a issues page
	- [ ] create a user settings page

- [ ] Actix backend
	- [x] login system
	- [x] ssl
	- [ ] change password
	- [ ] register an account
