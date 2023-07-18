# このツールについて

GitHub ReadMe Statsを参考に自分向けに修正を加えたツールです。

- GitHub Actionsによる定期実行を行いSVGファイルを生成します。
- 生成したSVGファイルをGitHubのプロフィールページにPUSHします。

ワークフローの詳細は`.github/workflows/publish.yaml`を参照してください。

# 開発の手順

## graphql_client_cliをインストールする

GraphQLのクエリを記述したファイルを元にRustのソースを生成するために`graphql_client_cli`をインストールします。

```
$ cargo install --git https://github.com/graphql-rust/graphql-client.git graphql_client_cli
```

インストール後、`graphql-client`コマンドを利用できるようになります。

## GitHub GraphQL APIのスキーマを生成する

下記コマンドを実行してスキーマファイルを生成します。変数${GITHUB_TOKEN}にGitHubのアカウント画面から発行したトークンを指定してください。

```
$ graphql-client introspect-schema https://api.github.com/graphql \
    --header "Authorization: bearer ${GITHUB_TOKEN}" \
    --header "user-agent: rust-graphql-client" > ./graphql/schema.json
```

## GraphQLのクエリファイルからRustのソースを生成する

GraphQLのクエリファイルを修正した場合、Rustのソースを再度生成してください。繰り返し実行するコマンドであるためMakefileにタスクが定義されています。下記のコマンドを実行してください。実行する際はトークンを記載した`github_pat`ファイルを配置してください。

```
$ make generate
```

## プログラムを実行しSVGファイルを生成する

```
$ make run
```

`image.svg`ファイルが生成されます。