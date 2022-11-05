import { Response, Request, Router } from "express";
import axios from "axios";
import config from "../config";

const router = Router({ mergeParams: true });

const GITHUB_HEADER = {
  headers: {
    Accept: "application/vnd.github+json",
    Authorization: `Bearer ${config.ACCESS_TOKEN}`,
  },
};

router.get("/active_projects", async (req: Request, res: Response) => {
  const x = await axios.post(
    "https://api.github.com/graphql",
    {
      query: `
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
    `,
      variables: {
        login: "kengo-k",
      },
    },
    GITHUB_HEADER
  );

  res.send(x.data);
});

export default router;
