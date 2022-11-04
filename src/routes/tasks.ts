import { Response, Request, Router } from "express";
import { Task } from "../models/task";
import { ChartJSNodeCanvas } from "chartjs-node-canvas";

const router = Router({ mergeParams: true });

const width = 400; //px
const height = 400; //px
const backgroundColour = "white";
const chartJSNodeCanvas = new ChartJSNodeCanvas({
  width,
  height,
  backgroundColour,
});

const createImage = async () => {
  const image = chartJSNodeCanvas.renderToBuffer({
    type: "line",
    data: {
      datasets: [
        {
          data: [0, 5, 200, 50, 20],
        },
        {
          data: [0, 1, 30, 2600, 300],
        },
      ],
    },
  });

  return image;
};

router.get("/", async (req: Request, res: Response) => {
  const image = await createImage();
  console.log("image: " + image);
  res.send(image);
});

export default router;
