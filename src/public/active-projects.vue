<template>
    <pie-chart :id="current.id" :title="current.title" :series="current.series" />
    <pie-chart :id="prev1.id" :title="prev1.title" :series="prev1.series" />
    <pie-chart :id="prev2.id" :title="prev2.title" :series="prev2.series" />
</template>

<script>
  import { startOfWeek, addWeeks, addSeconds, format } from "https://cdn.jsdelivr.net/npm/date-fns@2.29.3/index.min.js"

  const getDateRange = (num) => {
    const date = startOfWeek(new Date());
    const sinceDate = addWeeks(date, num);
    const untilDate = addSeconds(addWeeks(sinceDate, 1), -1);
    const tos = (d) => {
      return format(d, "yyyy-MM-dd");
    };
    return [tos(sinceDate), tos(untilDate)];
  };

  const createData = (json) => {
    return json.repos.map(r => {
      return {
        name: r.name, y:
        r.commit_count
      }
    })
  };

  export default {
    data: function() {
      console.log("[active-projects] data called")
      return {
        current: {
          id: "active-projects-current",
        },
        prev1: {
          id: "active-projects-prev1",
        },
        prev2: {
          id: "active-projects-prev2",
        },
      }
    },
    methods: {
      init(json, since, until, chartData) {
        const data = createData(json);
        chartData.series = data;
        chartData.title = `from ${since} to ${until}`
      }
    },
    mounted() {
      console.log("[active-projects] mounted called")
      const currentRange = getDateRange(0);
      const prev1Range = getDateRange(-1);
      const prev2Range = getDateRange(-2);

      fetch(`/api/active_projects?since=${currentRange[0]}&until=${currentRange[1]}`)
        .then((response) => response.json())
        .then((json) => this.init(json, currentRange[0], currentRange[1], this.current));

      fetch(`/api/active_projects?since=${prev1Range[0]}&until=${prev1Range[1]}`)
        .then((response) => response.json())
        .then((json) => this.init(json, prev1Range[0], prev1Range[1], this.prev1));

      fetch(`/api/active_projects?since=${prev2Range[0]}&until=${prev2Range[1]}`)
        .then((response) => response.json())
        .then((json) => this.init(json, prev2Range[0], prev2Range[1], this.prev2));
    }
  }
</script>
