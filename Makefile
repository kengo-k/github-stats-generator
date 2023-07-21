build:
	cargo watch -x build

run:
	GITHUB_TOKEN=$$(cat ./github_pat) cargo run

generate:
	graphql-client generate -p crate::graphql::custom_scalars -o src/generated --schema-path graphql/schema.json graphql/github_stats.graphql

server:
	npx http-server -c-1 --cors
