build:
	cargo build

dev:
	cargo watch -x run

server:
	npx http-server -c-1 --cors
