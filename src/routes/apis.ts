import { Response, Request, Router } from "express";
import config from "../config";
import { callNodesQuery } from "../graphql/nodes";
import { callUserRepositoriesQuery } from "../graphql/user_repositories";
import {
  startOfWeek,
  format,
  startOfMonth,
  addDays,
  addMonths,
} from "date-fns";
import { callLangsQuery } from "../graphql/langs";

const router = Router({ mergeParams: true });

const getDateRange = (): [string, string] => {
  const sinceDate = startOfMonth(new Date());
  const untilDate = addDays(addMonths(sinceDate, 1), -1);
  const tos = (d: Date, isEnd: boolean) => {
    const ymd = format(d, "yyyy-MM-dd");
    let timeZone = "T00:00:00+09:00";
    if (isEnd) {
      timeZone = "T23:59:59+09:00";
    }
    return `${ymd}${timeZone}`;
  };
  return [tos(sinceDate, false), tos(untilDate, true)];
};

router.get("/langs", async (req: Request, res: Response) => {
  const IGNORES = ["Dockerfile", "Makefile", "Nix", "Vim Script", "HTML"];
  const MAPPING: { [key: string]: string } = {
    MQL4: "MQL",
    MQL5: "MQL",
    CSS: "CSS/SCSS",
    SCSS: "CSS/SCSS",
  };
  const langs = await callLangsQuery({ login: config.OWNER });
  const nodes = langs.data.data.user.repositories.nodes;
  const langMap: {
    [key: string]: {
      size: number;
      color?: string;
      details: {
        [key: string]: number;
      };
    };
  } = {};
  let totalSize = 0;
  nodes.forEach((n: any) => {
    const edges = n.languages.edges;
    edges.forEach((e: any) => {
      let langName: string = e.node.name;
      if (IGNORES.includes(langName)) {
        return;
      }
      if (langName in MAPPING) {
        langName = MAPPING[langName];
      }
      if (!langMap[langName]) {
        langMap[langName] = { size: 0, details: {} };
      }
      totalSize += e.size;
      langMap[langName]["size"] += e.size;
      langMap[langName]["details"][n.name] = e.size;
      langMap[langName]["color"] = e.node.color;
    });
    return { name: n.name, langs: langMap };
  });
  res.send({ totalSize, languages: langMap });
});

router.get("/active_projects", async (req: Request, res: Response) => {
  const [since, until] = getDateRange();

  // リポジトリの一覧を取得
  const projects = await callUserRepositoriesQuery({ login: config.OWNER });
  // TODO 直近でPUSHされたものだけを抽出
  const repoIDs = projects.data.data.user.repositories.nodes
    .filter((p: any) => {
      return p;
    })
    .map((p: any) => {
      return p.id;
    });

  const recentRepos = await callNodesQuery(since, until, { ids: repoIDs });
  const ret = recentRepos.data.data.nodes
    .filter((r: any) => {
      if (r.defaultBranchRef != null) {
        const totalCount = r.defaultBranchRef.target.history.totalCount;
        return totalCount > 0;
      }
    })
    .map((r: any) => {
      return {
        name: r.name,
        commit_count: r.defaultBranchRef.target.history.totalCount,
        langs: r.languages.edges.flatMap((e: any) => {
          return [e.node.name];
        }),
      };
    });
  let total_commit_count = 0;
  ret.forEach((r: any) => {
    total_commit_count += r.commit_count;
  });
  res.send(Object.assign({}, { since, until, total_commit_count, repos: ret }));
});

export default router;
