import { Router } from "express";
import tasks from "./tasks";

const router = Router({ mergeParams: true });
router.use("/api", tasks);

export default router;
