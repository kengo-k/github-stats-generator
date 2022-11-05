import { Router } from "express";
import apis from "./apis";

const router = Router({ mergeParams: true });
router.use("/api", apis);

export default router;
