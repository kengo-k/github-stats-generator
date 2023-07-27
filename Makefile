build:
	cargo watch -x build

run:
	GITHUB_TOKEN=$$(cat ./github_pat) cargo watch -x run

test:
	GITHUB_TOKEN=$$(cat ./github_pat) cargo watch -x test

generate:
	graphql-client generate -p crate::graphql::custom_scalars -o src/generated --schema-path graphql/schema.json graphql/github_stats.graphql

server:
	npx http-server -c-1 --cors
