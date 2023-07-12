#![allow(clippy::all, warnings)]
pub struct GithubStats;
pub mod github_stats {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GithubStats";
    pub const QUERY : & str = "query GithubStats {\n  viewer {\n    repositories(first: 100) {\n      edges {\n        node {\n          name\n          languages(first: 100) {\n            edges {\n              size\n              node {\n                name\n                color\n              }\n            }\n          }\n        }\n      }\n    }\n  }\n}\n" ;
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
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub viewer: GithubStatsViewer,
    }
    #[derive(Deserialize)]
    pub struct GithubStatsViewer {
        pub repositories: GithubStatsViewerRepositories,
    }
    #[derive(Deserialize)]
    pub struct GithubStatsViewerRepositories {
        pub edges: Option<Vec<Option<GithubStatsViewerRepositoriesEdges>>>,
    }
    #[derive(Deserialize)]
    pub struct GithubStatsViewerRepositoriesEdges {
        pub node: Option<GithubStatsViewerRepositoriesEdgesNode>,
    }
    #[derive(Deserialize)]
    pub struct GithubStatsViewerRepositoriesEdgesNode {
        pub name: String,
        pub languages: Option<GithubStatsViewerRepositoriesEdgesNodeLanguages>,
    }
    #[derive(Deserialize)]
    pub struct GithubStatsViewerRepositoriesEdgesNodeLanguages {
        pub edges: Option<Vec<Option<GithubStatsViewerRepositoriesEdgesNodeLanguagesEdges>>>,
    }
    #[derive(Deserialize)]
    pub struct GithubStatsViewerRepositoriesEdgesNodeLanguagesEdges {
        pub size: Int,
        pub node: GithubStatsViewerRepositoriesEdgesNodeLanguagesEdgesNode,
    }
    #[derive(Deserialize)]
    pub struct GithubStatsViewerRepositoriesEdgesNodeLanguagesEdgesNode {
        pub name: String,
        pub color: Option<String>,
    }
}
impl graphql_client::GraphQLQuery for GithubStats {
    type Variables = github_stats::Variables;
    type ResponseData = github_stats::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: github_stats::QUERY,
            operation_name: github_stats::OPERATION_NAME,
        }
    }
}
