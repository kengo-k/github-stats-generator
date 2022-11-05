import { Response, Request, Router } from "express";
import { ActiveProjects, Project } from "../models/task";
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

  // // list all repositories
  // const repos = await axios.get(
  //   `https://api.github.com/users/${config.OWNER}/repos`,
  //   GITHUB_HEADER
  // );

  // // get commit count per repository in this week
  // let totalCommits = 0;
  // const projects: Project[] = [];
  // for (const repo of repos.data) {
  //   const commits = await axios.get(
  //     `https://api.github.com/repos/${config.OWNER}/${repo.name}/commits?since=2022-11-01T15:00:00.00&until=2022-11-02T15:00:00.00`,
  //     GITHUB_HEADER
  //   );
  //   const p: Project = {
  //     repository_name: repo.name,
  //     commit_count: commits.data.length,
  //   };
  //   totalCommits += commits.data.length;
  //   projects.push(p);
  // }

  // const data: ActiveProjects = {
  //   from_date: "2022-11-04",
  //   to_date: "2022-11-04",
  //   projects: projects.map((p) => {
  //     return Object.assign({}, p, {
  //       ratio: (p.commit_count / totalCommits) * 100,
  //     });
  //   }),
  // };
  res.send(x.data);
});

export default router;
