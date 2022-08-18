test:
	cd yew-frontend && cargo test
	cd actix-backend && cargo test

serve:
	cd yew-frontend && trunk serve --open

start:
	bash -c "./start.sh"
