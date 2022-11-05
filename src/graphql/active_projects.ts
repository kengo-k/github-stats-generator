import { callGraphQL } from "../services/github";

export const ActiveProjectsQuery = `
  query user($login: String!) {
    user(login: $login) {
      repositories(
        first: 100
        orderBy: {field: PUSHED_AT, direction: DESC}
      ) {
        nodes {
          id
          name
          isPrivate
          pushedAt
        }
      }
    }
  }
`;

export interface ActiveProjectsQueryVariableType {
  login: string;
}

export const callActiveProjectsQuery = (
  variables: ActiveProjectsQueryVariableType
) => {
  return callGraphQL<typeof variables>(ActiveProjectsQuery, variables);
};
