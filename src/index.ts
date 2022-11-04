import { json, urlencoded } from "body-parser";
import express from "express";
import config from "./config";
import routes from "./routes";

const app = express();

app.use(urlencoded({ extended: true }));
app.use("/public", express.static(__dirname + "/public"));
app.use(json());
app.use(routes);

app.listen(config.port, () => console.log(`Listening on port: ${config.port}`));
