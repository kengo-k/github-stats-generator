import { Response, Request, Router } from "express";
import config from "../config";
import { callNodesQuery } from "../graphql/nodes";
import { callUserRepositoriesQuery } from "../graphql/user_repositories";

const router = Router({ mergeParams: true });

router.get("/active_projects", async (req: Request, res: Response) => {
  const projects = await callUserRepositoriesQuery({ login: config.OWNER });
  // 指定期間内にPUSHされたリポジトリの一覧のみを抽出
  const recentProjectIDs = projects.data.data.user.repositories.nodes
    .filter((p: any) => {
      return p;
    })
    .map((p: any) => {
      return p.id;
    });
  const x = await callNodesQuery({ ids: recentProjectIDs });
  res.send(x.data);
});

export default router;
