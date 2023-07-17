#![allow(clippy::all, warnings)]
pub struct ListRepositories;
pub mod list_repositories {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ListRepositories";
    pub const QUERY : & str = "query ListRepositories($login: String!) {\n  user(login: $login) {\n    repositories(first: 100, orderBy: { field: PUSHED_AT, direction: DESC }) {\n      nodes {\n        id\n        name\n        isPrivate\n        isFork\n        diskUsage\n        pushedAt\n      }\n    }\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type DateTime = super::DateTime;
    #[derive(Serialize)]
    pub struct Variables {
        pub login: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub user: Option<ListRepositoriesUser>,
    }
    #[derive(Deserialize)]
    pub struct ListRepositoriesUser {
        pub repositories: ListRepositoriesUserRepositories,
    }
    #[derive(Deserialize)]
    pub struct ListRepositoriesUserRepositories {
        pub nodes: Option<Vec<Option<ListRepositoriesUserRepositoriesNodes>>>,
    }
    #[derive(Deserialize)]
    pub struct ListRepositoriesUserRepositoriesNodes {
        pub id: ID,
        pub name: String,
        #[serde(rename = "isPrivate")]
        pub is_private: Boolean,
        #[serde(rename = "isFork")]
        pub is_fork: Boolean,
        #[serde(rename = "diskUsage")]
        pub disk_usage: Option<Int>,
        #[serde(rename = "pushedAt")]
        pub pushed_at: Option<DateTime>,
    }
}
impl graphql_client::GraphQLQuery for ListRepositories {
    type Variables = list_repositories::Variables;
    type ResponseData = list_repositories::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: list_repositories::QUERY,
            operation_name: list_repositories::OPERATION_NAME,
        }
    }
}
