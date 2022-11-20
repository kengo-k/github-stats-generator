import { json, urlencoded } from "body-parser";
import express from "express";
import config from "./config";
import routes from "./routes";

const app = express();

app.use(urlencoded({ extended: true }));
app.use("/", express.static(__dirname + "/public"));
app.use(json());
app.use(routes);

app.listen(config.PORT, () => console.log(`Listening on port: ${config.PORT}`));
