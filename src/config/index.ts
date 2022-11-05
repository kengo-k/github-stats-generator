import { config } from "dotenv";

config();

export interface ConfigType {
  PORT: number;
  ACCESS_TOKEN: string;
  OWNER: string;
}

const configValue = {
  PORT: process.env.PORT || 3000,
  ACCESS_TOKEN: process.env.ACCESS_TOKEN,
  OWNER: process.env.OWNER,
};

if (configValue.ACCESS_TOKEN == null) {
  throw new Error("missing environment: ACCESS_TOKEN");
}

if (configValue.OWNER == null) {
  throw new Error("missing environment: OWNER");
}

export default configValue as any as Required<ConfigType>;
