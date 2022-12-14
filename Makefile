test:
	cd yew-frontend && cargo test
	cd actix-backend && cargo test

serve-frontend:
	cd yew-frontend && dotenv-rust -f .env trunk serve

update-css:
	cd yew-frontend && cargo watch -- cp -rv styles dist/

serve-backend:
	cd actix-backend && dotenv-rust -f .env cargo watch -x "run"

start:
	bash -c "./start.sh"
