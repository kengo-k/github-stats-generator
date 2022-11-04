import { Response, Request, Router } from "express";
import { ActiveProjects } from "../models/task";

const router = Router({ mergeParams: true });

router.get("/active_projects", async (req: Request, res: Response) => {
  const data: ActiveProjects = {
    from_date: "2022-11-04",
    to_date: "2022-11-04",
    projects: [],
  };
  res.send(data);
});

export default router;
