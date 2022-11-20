import config from "../config";
import axios from "axios";

const GITHUB_HEADER = {
  headers: {
    Accept: "application/vnd.github+json",
    Authorization: `Bearer ${config.ACCESS_TOKEN}`,
  },
};

export const callGraphQL = async <T>(query: string, variables: T) => {
  return await axios.post(
    "https://api.github.com/graphql",
    {
      query,
      variables,
    },
    GITHUB_HEADER
  );
};
