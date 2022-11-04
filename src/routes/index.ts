import { Router } from "express";
import tasks from "./tasks";

const router = Router({ mergeParams: true });
router.use("/images", tasks);

export default router;
