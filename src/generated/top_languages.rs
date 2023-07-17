#![allow(clippy::all, warnings)]
pub struct TopLanguages;
pub mod top_languages {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "TopLanguages";
    pub const QUERY : & str = "query TopLanguages {\n  viewer {\n    repositories(isFork: false, first: 100) {\n      edges {\n        node {\n          name\n          stargazerCount\n          languages(first: 100) {\n            edges {\n              size\n              node {\n                name\n                color\n              }\n            }\n          }\n        }\n      }\n    }\n  }\n}\n" ;
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
        pub viewer: TopLanguagesViewer,
    }
    #[derive(Deserialize)]
    pub struct TopLanguagesViewer {
        pub repositories: TopLanguagesViewerRepositories,
    }
    #[derive(Deserialize)]
    pub struct TopLanguagesViewerRepositories {
        pub edges: Option<Vec<Option<TopLanguagesViewerRepositoriesEdges>>>,
    }
    #[derive(Deserialize)]
    pub struct TopLanguagesViewerRepositoriesEdges {
        pub node: Option<TopLanguagesViewerRepositoriesEdgesNode>,
    }
    #[derive(Deserialize)]
    pub struct TopLanguagesViewerRepositoriesEdgesNode {
        pub name: String,
        #[serde(rename = "stargazerCount")]
        pub stargazer_count: Int,
        pub languages: Option<TopLanguagesViewerRepositoriesEdgesNodeLanguages>,
    }
    #[derive(Deserialize)]
    pub struct TopLanguagesViewerRepositoriesEdgesNodeLanguages {
        pub edges: Option<Vec<Option<TopLanguagesViewerRepositoriesEdgesNodeLanguagesEdges>>>,
    }
    #[derive(Deserialize)]
    pub struct TopLanguagesViewerRepositoriesEdgesNodeLanguagesEdges {
        pub size: Int,
        pub node: TopLanguagesViewerRepositoriesEdgesNodeLanguagesEdgesNode,
    }
    #[derive(Deserialize)]
    pub struct TopLanguagesViewerRepositoriesEdgesNodeLanguagesEdgesNode {
        pub name: String,
        pub color: Option<String>,
    }
}
impl graphql_client::GraphQLQuery for TopLanguages {
    type Variables = top_languages::Variables;
    type ResponseData = top_languages::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: top_languages::QUERY,
            operation_name: top_languages::OPERATION_NAME,
        }
    }
}
