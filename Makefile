test:
	cd yew-frontend && cargo test
	cd actix-backend && cargo test

serve-frontend:
	cd yew-frontend && dotenv-rust -f .env trunk serve

update-css-both:
	/bin/sh -c "cp -rv yew-frontend/styles yew-frontend/dist/" & \
	/bin/sh -c "cp -rv yew-frontend/styles actix-backend/ &" & \
	wait;

update-css:
	cd yew-frontend && cargo watch -- make -C ../ update-css-both

serve-backend:
	cd actix-backend && dotenv-rust -f .env cargo watch -x "run"

start:
	bash -c "./start.sh"
