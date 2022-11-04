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
      labels: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"],
      datasets: [
        {
          label: "Red",
          data: [20, 35, 40, 30, 45, 35, 40],
          borderColor: "#f88",
        },
        {
          label: "Green",
          data: [20, 15, 30, 25, 30, 40, 35],
          borderColor: "#484",
        },
        {
          label: "Blue",
          data: [30, 25, 10, 5, 25, 30, 20],
          borderColor: "#48f",
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
