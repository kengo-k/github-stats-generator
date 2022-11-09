import { callGraphQL } from "../services/github";

export const LangsQuery = `
  query langs($login: String!) {
    user(login: $login) {
      repositories(first: 100) {
        nodes {
          name
          languages(first: 100) {
            edges {
              node {
                name
                color
              }
              size
            }
          }
        }
        pageInfo {
          hasNextPage
          startCursor
          endCursor
        }
        totalCount
      }
    }
  }
`;

export interface LangsQueryVariableType {
  login: string;
}

export const callLangsQuery = (variables: LangsQueryVariableType) => {
  return callGraphQL<typeof variables>(LangsQuery, variables);
};
