build:
	cargo build

dev:
	cargo watch -x run

generate:
	graphql-client generate -o src --schema-path graphql/schema.json graphql/query.graphql

server:
	npx http-server -c-1 --cors

