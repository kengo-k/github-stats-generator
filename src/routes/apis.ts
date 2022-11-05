import { Response, Request, Router } from "express";
import config from "../config";
import { callActiveProjectsQuery } from "../graphql/active_projects";

const router = Router({ mergeParams: true });

router.get("/active_projects", async (req: Request, res: Response) => {
  const activeProjects = await callActiveProjectsQuery({ login: config.OWNER });
  res.send(activeProjects.data);
});

export default router;
