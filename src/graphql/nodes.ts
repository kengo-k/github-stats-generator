import { callGraphQL } from "../services/github";

export const NodesQuery = `
  query nodes($ids: [ID!]!) {
    nodes(ids: $ids) {
      ... on Repository {
        name
        defaultBranchRef {
          target {
            ... on Commit {
              history(since: "2022-11-04T00:00:00.00") {
                totalCount
              }
            }
          }
        }
      }
    }
  }
`;

export interface NodesQueryVariableType {
  ids: string[];
}

export const callNodesQuery = (variables: NodesQueryVariableType) => {
  return callGraphQL<typeof variables>(NodesQuery, variables);
};
