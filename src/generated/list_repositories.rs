#![allow(clippy::all, warnings)]
pub struct ListRepositories;
pub mod list_repositories {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ListRepositories";
    pub const QUERY : & str = "# import_types: chrono::DateTime<chrono::Utc>\n# as_types: DateTime -> chrono::DateTime<chrono::Utc>\n\nquery ListRepositories {\n  viewer {\n    repositories(first: 100, orderBy: { field: PUSHED_AT, direction: DESC }) {\n      nodes {\n        id\n        name\n        isPrivate\n        isFork\n        isArchived\n        isTemplate\n        diskUsage\n        stargazerCount\n        pushedAt\n        repositoryTopics(first: 10) {\n          edges {\n            node {\n              topic {\n                name\n              }\n            }\n          }\n        }\n      }\n    }\n  }\n}\n" ;
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
    type DateTime = crate::graphql::custom_scalars::DateTime;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub viewer: ListRepositoriesViewer,
    }
    #[derive(Deserialize)]
    pub struct ListRepositoriesViewer {
        pub repositories: ListRepositoriesViewerRepositories,
    }
    #[derive(Deserialize)]
    pub struct ListRepositoriesViewerRepositories {
        pub nodes: Option<Vec<Option<ListRepositoriesViewerRepositoriesNodes>>>,
    }
    #[derive(Deserialize)]
    pub struct ListRepositoriesViewerRepositoriesNodes {
        pub id: ID,
        pub name: String,
        #[serde(rename = "isPrivate")]
        pub is_private: Boolean,
        #[serde(rename = "isFork")]
        pub is_fork: Boolean,
        #[serde(rename = "isArchived")]
        pub is_archived: Boolean,
        #[serde(rename = "isTemplate")]
        pub is_template: Boolean,
        #[serde(rename = "diskUsage")]
        pub disk_usage: Option<Int>,
        #[serde(rename = "stargazerCount")]
        pub stargazer_count: Int,
        #[serde(rename = "pushedAt")]
        pub pushed_at: Option<DateTime>,
        #[serde(rename = "repositoryTopics")]
        pub repository_topics: ListRepositoriesViewerRepositoriesNodesRepositoryTopics,
    }
    #[derive(Deserialize)]
    pub struct ListRepositoriesViewerRepositoriesNodesRepositoryTopics {
        pub edges:
            Option<Vec<Option<ListRepositoriesViewerRepositoriesNodesRepositoryTopicsEdges>>>,
    }
    #[derive(Deserialize)]
    pub struct ListRepositoriesViewerRepositoriesNodesRepositoryTopicsEdges {
        pub node: Option<ListRepositoriesViewerRepositoriesNodesRepositoryTopicsEdgesNode>,
    }
    #[derive(Deserialize)]
    pub struct ListRepositoriesViewerRepositoriesNodesRepositoryTopicsEdgesNode {
        pub topic: ListRepositoriesViewerRepositoriesNodesRepositoryTopicsEdgesNodeTopic,
    }
    #[derive(Deserialize)]
    pub struct ListRepositoriesViewerRepositoriesNodesRepositoryTopicsEdgesNodeTopic {
        pub name: String,
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
