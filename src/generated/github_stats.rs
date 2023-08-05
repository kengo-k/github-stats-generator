#![allow(clippy::all, warnings)]
pub struct GitHubStats;
pub mod git_hub_stats {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GitHubStats";
    pub const QUERY : & str = "query GitHubStats($from: GitTimestamp!, $to: GitTimestamp!) {\n  viewer {\n    repositories(isFork: false, first: 100) {\n      nodes {\n        id\n        name\n        isPrivate\n        isFork\n        isArchived\n        isTemplate\n        diskUsage\n        stargazerCount\n        pushedAt\n        repositoryTopics(first: 100) {\n          edges {\n            node {\n              topic {\n                name\n              }\n            }\n          }\n        }\n        languages(first: 100) {\n          edges {\n            node {\n              name\n              color\n            }\n            size\n          }\n        }\n        defaultBranchRef {\n          target {\n            __typename\n            ... on Commit {\n              commitHistoryPeriod: history(since: $from, until: $to) {\n                totalCount\n              }\n              commitHistoryAll: history {\n                totalCount\n              }\n            }\n          }\n        }\n      }\n    }\n  }\n}\n" ;
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
    type GitTimestamp = crate::graphql::custom_scalars::GitTimestamp;
    #[derive(Serialize, Debug)]
    pub struct Variables {
        pub from: GitTimestamp,
        pub to: GitTimestamp,
    }
    impl Variables {}
    #[derive(Deserialize, Serialize, Debug)]
    pub struct ResponseData {
        pub viewer: GitHubStatsViewer,
    }
    #[derive(Deserialize, Serialize, Debug)]
    pub struct GitHubStatsViewer {
        pub repositories: GitHubStatsViewerRepositories,
    }
    #[derive(Deserialize, Serialize, Debug)]
    pub struct GitHubStatsViewerRepositories {
        pub nodes: Option<Vec<Option<GitHubStatsViewerRepositoriesNodes>>>,
    }
    #[derive(Deserialize, Serialize, Debug)]
    pub struct GitHubStatsViewerRepositoriesNodes {
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
        pub repository_topics: GitHubStatsViewerRepositoriesNodesRepositoryTopics,
        pub languages: Option<GitHubStatsViewerRepositoriesNodesLanguages>,
        #[serde(rename = "defaultBranchRef")]
        pub default_branch_ref: Option<GitHubStatsViewerRepositoriesNodesDefaultBranchRef>,
    }
    #[derive(Deserialize, Serialize, Debug)]
    pub struct GitHubStatsViewerRepositoriesNodesRepositoryTopics {
        pub edges: Option<Vec<Option<GitHubStatsViewerRepositoriesNodesRepositoryTopicsEdges>>>,
    }
    #[derive(Deserialize, Serialize, Debug)]
    pub struct GitHubStatsViewerRepositoriesNodesRepositoryTopicsEdges {
        pub node: Option<GitHubStatsViewerRepositoriesNodesRepositoryTopicsEdgesNode>,
    }
    #[derive(Deserialize, Serialize, Debug)]
    pub struct GitHubStatsViewerRepositoriesNodesRepositoryTopicsEdgesNode {
        pub topic: GitHubStatsViewerRepositoriesNodesRepositoryTopicsEdgesNodeTopic,
    }
    #[derive(Deserialize, Serialize, Debug)]
    pub struct GitHubStatsViewerRepositoriesNodesRepositoryTopicsEdgesNodeTopic {
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug)]
    pub struct GitHubStatsViewerRepositoriesNodesLanguages {
        pub edges: Option<Vec<Option<GitHubStatsViewerRepositoriesNodesLanguagesEdges>>>,
    }
    #[derive(Deserialize, Serialize, Debug)]
    pub struct GitHubStatsViewerRepositoriesNodesLanguagesEdges {
        pub node: GitHubStatsViewerRepositoriesNodesLanguagesEdgesNode,
        pub size: Int,
    }
    #[derive(Deserialize, Serialize, Debug)]
    pub struct GitHubStatsViewerRepositoriesNodesLanguagesEdgesNode {
        pub name: String,
        pub color: Option<String>,
    }
    #[derive(Deserialize, Serialize, Debug)]
    pub struct GitHubStatsViewerRepositoriesNodesDefaultBranchRef {
        pub target: Option<GitHubStatsViewerRepositoriesNodesDefaultBranchRefTarget>,
    }
    #[derive(Deserialize, Serialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum GitHubStatsViewerRepositoriesNodesDefaultBranchRefTarget {
        Blob,
        Commit(GitHubStatsViewerRepositoriesNodesDefaultBranchRefTargetOnCommit),
        Tag,
        Tree,
    }
    #[derive(Deserialize, Serialize, Debug)]
    pub struct GitHubStatsViewerRepositoriesNodesDefaultBranchRefTargetOnCommit {
        #[serde(rename = "commitHistoryPeriod")]
        pub commit_history_period:
            GitHubStatsViewerRepositoriesNodesDefaultBranchRefTargetOnCommitCommitHistoryPeriod,
        #[serde(rename = "commitHistoryAll")]
        pub commit_history_all:
            GitHubStatsViewerRepositoriesNodesDefaultBranchRefTargetOnCommitCommitHistoryAll,
    }
    #[derive(Deserialize, Serialize, Debug)]
    pub struct GitHubStatsViewerRepositoriesNodesDefaultBranchRefTargetOnCommitCommitHistoryPeriod {
        #[serde(rename = "totalCount")]
        pub total_count: Int,
    }
    #[derive(Deserialize, Serialize, Debug)]
    pub struct GitHubStatsViewerRepositoriesNodesDefaultBranchRefTargetOnCommitCommitHistoryAll {
        #[serde(rename = "totalCount")]
        pub total_count: Int,
    }
}
impl graphql_client::GraphQLQuery for GitHubStats {
    type Variables = git_hub_stats::Variables;
    type ResponseData = git_hub_stats::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: git_hub_stats::QUERY,
            operation_name: git_hub_stats::OPERATION_NAME,
        }
    }
}
