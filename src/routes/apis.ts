import { Response, Request, Router } from "express";
import config from "../config";
import { callNodesQuery } from "../graphql/nodes";
import { callUserRepositoriesQuery } from "../graphql/user_repositories";

const router = Router({ mergeParams: true });

router.get("/active_projects", async (req: Request, res: Response) => {
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
  const recentRepos = await callNodesQuery({ ids: repoIDs });
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
      };
    });
  res.send(ret);
});

export default router;
