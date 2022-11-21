<template>
  <PieChart :id="current.id" :title="current.title" :series="current.series" />
  <PieChart :id="prev1.id" :title="prev1.title" :series="prev1.series" />
  <PieChart :id="prev2.id" :title="prev2.title" :series="prev2.series" />
</template>

<script lang="ts">
import { defineComponent } from "vue";

import { addSeconds, addWeeks, format, startOfWeek } from "date-fns";
import { ChartData, ChartSeriesData } from "./charts/PieChart.vue";
import PieChart from "./charts/PieChart.vue";

interface ResponseData {
  repos: {
    name: string;
    commit_count: number;
  }[];
}

const getDateRange = (num: number) => {
  const date = startOfWeek(new Date());
  const sinceDate = addWeeks(date, num);
  const untilDate = addSeconds(addWeeks(sinceDate, 1), -1);
  const tos = (d: Date) => {
    return format(d, "yyyy-MM-dd");
  };
  return [tos(sinceDate), tos(untilDate)];
};

const createData = (json: ResponseData): ChartSeriesData[] => {
  return json.repos.map((r) => {
    return {
      name: r.name,
      y: r.commit_count,
    };
  });
};

export default defineComponent({
  name: "ActiveProjects",
  components: { PieChart },
  data: function () {
    console.log("[active-projects] data called");
    return {
      current: {
        id: "active-projects-current",
        title: "",
        series: [],
      },
      prev1: {
        id: "active-projects-prev1",
        title: "",
        series: [],
      },
      prev2: {
        id: "active-projects-prev2",
        title: "",
        series: [],
      },
    };
  },
  methods: {
    init(
      json: ResponseData,
      since: string,
      until: string,
      chartData: ChartData
    ) {
      const data = createData(json);
      chartData.series = data;
      chartData.title = `from ${since} to ${until}`;
    },
  },
  mounted() {
    console.log("[active-projects] mounted called");
    const currentRange = getDateRange(0);
    const prev1Range = getDateRange(-1);
    const prev2Range = getDateRange(-2);

    fetch(
      `/api/active_projects?since=${currentRange[0]}&until=${currentRange[1]}`
    )
      .then((response) => response.json())
      .then((json) =>
        this.init(json, currentRange[0], currentRange[1], this.current)
      );

    fetch(`/api/active_projects?since=${prev1Range[0]}&until=${prev1Range[1]}`)
      .then((response) => response.json())
      .then((json) =>
        this.init(json, prev1Range[0], prev1Range[1], this.prev1)
      );

    fetch(`/api/active_projects?since=${prev2Range[0]}&until=${prev2Range[1]}`)
      .then((response) => response.json())
      .then((json) =>
        this.init(json, prev2Range[0], prev2Range[1], this.prev2)
      );
  },
});
</script>
