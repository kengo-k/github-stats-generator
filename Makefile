build:
	cargo watch -x build

run:
	GITHUB_TOKEN=$$(cat ./github_pat) cargo run

generate:
	graphql-client generate -o src/generated --schema-path graphql/schema.json graphql/top_languages.graphql
	graphql-client generate -p crate::graphql::custom_scalars -o src/generated --schema-path graphql/schema.json graphql/list_repositories.graphql
	graphql-client generate -o src/generated --schema-path graphql/schema.json graphql/active_repositories.graphql

server:
	npx http-server -c-1 --cors

