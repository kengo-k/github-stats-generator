# How to Run

```
$ cargo run
```

#

## GraphQLのCLIツールであるgraphql-clientコマンドをインストールする

```
$ cargo install --git https://github.com/graphql-rust/graphql-client.git graphql_client_cli
```

##

```
$ graphql-client introspect-schema https://api.github.com/graphql \
--header "Authorization: bearer ${GH_PAT}" \
--header "user-agent: rust-graphql-client" > ./schema.json
```

```
$ graphql-client generate -o src --schema-path graphql/schema.json graphql/query.graphql
```
