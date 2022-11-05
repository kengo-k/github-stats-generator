import { config } from "dotenv";

config();

export default {
  port: process.env.PORT || 3000,
  ACCESS_TOKEN: process.env.ACCESS_TOKEN,
  OWNER: process.env.OWNER,
};
