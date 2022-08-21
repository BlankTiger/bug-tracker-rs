test:
	cd yew-frontend && cargo test
	cd actix-backend && cargo test

serve-front:
	cd yew-frontend && source .env && trunk serve

update-css:
	cd yew-frontend && cargo watch -- cp -rv styles dist/

serve-back:
	cd actix-backend && source .env && cargo watch -x "run"

start:
	bash -c "./start.sh"
