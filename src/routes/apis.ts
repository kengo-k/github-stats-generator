import { Response, Request, Router } from "express";
import config from "../config";
import { callNodesQuery } from "../graphql/nodes";
import { callUserRepositoriesQuery } from "../graphql/user_repositories";
import { startOfWeek, format } from "date-fns";
import { callLangsQuery } from "../graphql/langs";

const router = Router({ mergeParams: true });

const getStartOfWeek = (): string => {
  const date = startOfWeek(new Date());
  const ymd = format(date, "yyyy-MM-dd");
  const timeZone = "T00:00:00+09:00";
  return `${ymd}${timeZone}`;
};

router.get("/langs", async (req: Request, res: Response) => {
  const langs = await callLangsQuery({ login: config.OWNER });
  const nodes = langs.data.data.user.repositories.nodes;
  res.send(nodes);
});

router.get("/active_projects", async (req: Request, res: Response) => {
  const since = getStartOfWeek();

  // リポジトリの一覧を取得
  const projects = await callUserRepositoriesQuery({ login: config.OWNER });
  // TODO 直近でPUSHされたものだけを抽出
  const repoIDs = projects.data.data.user.repositories.nodes
    .filter((p: any) => {
      return p;
    })
    .map((p: any) => {
      return p.id;
    });

  const recentRepos = await callNodesQuery(since, { ids: repoIDs });
  const ret = recentRepos.data.data.nodes
    .filter((r: any) => {
      if (r.defaultBranchRef != null) {
        const totalCount = r.defaultBranchRef.target.history.totalCount;
        return totalCount > 0;
      }
    })
    .map((r: any) => {
      return {
        name: r.name,
        commit_count: r.defaultBranchRef.target.history.totalCount,
        langs: r.languages.edges.flatMap((e: any) => {
          return [e.node.name];
        }),
      };
    });
  let total_commit_count = 0;
  ret.forEach((r: any) => {
    total_commit_count += r.commit_count;
  });
  res.send(Object.assign({}, { since, total_commit_count, repos: ret }));
});

export default router;
