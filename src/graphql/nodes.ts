import { callGraphQL } from "../services/github";

export const NodesQuery = (since: string) => `
  query nodes($ids: [ID!]!) {
    nodes(ids: $ids) {
      ... on Repository {
        name
        defaultBranchRef {
          target {
            ... on Commit {
              history(since: "${since}") {
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

export const callNodesQuery = (
  since: string,
  variables: NodesQueryVariableType
) => {
  return callGraphQL<typeof variables>(NodesQuery(since), variables);
};
