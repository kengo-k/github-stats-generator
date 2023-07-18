# About This Tool

This tool, which has been customized for personal use, draws inspiration from GitHub ReadMe Stats.

- It periodically generates SVG files using GitHub Actions.
- These generated SVG files are pushed to your GitHub profile page.
  
Please refer to `.github/workflows/publish.yaml` for more details about the workflow.

# How to Develop

## Install graphql_client_cli

We use `graphql_client_cli` to generate Rust sources from GraphQL query files.

```
$ cargo install --git https://github.com/graphql-rust/graphql-client.git graphql_client_cli
```

Once installed, the `graphql-client` command will become available.

## Generate GitHub GraphQL API Schema

Run the command below to generate the schema file. The variable ${GITHUB_TOKEN} should be assigned a token that you've generated from your GitHub account settings.

```
$ graphql-client introspect-schema https://api.github.com/graphql \
    --header "Authorization: bearer ${GITHUB_TOKEN}" \
    --header "user-agent: rust-graphql-client" > ./graphql/schema.json
```

## Generate Rust Source Code from GraphQL Query File

If you modify the GraphQL query file, you will need to regenerate the Rust source code. This process is repeatable, and a task has been defined in the Makefile. Run the command below. Remember to place the `github_pat` file, which contains the token, in the correct location before executing.

```
$ make generate
```

## Run the Program to Generate SVG File

```
$ make run
```

This command will generate an image.svg file.
