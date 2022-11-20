import { callGraphQL } from "../services/github";

export const UserRepositoriesQuery = `
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

export interface UserRepositoriesQueryVariableType {
  login: string;
}

export const callUserRepositoriesQuery = (
  variables: UserRepositoriesQueryVariableType
) => {
  return callGraphQL<typeof variables>(UserRepositoriesQuery, variables);
};
