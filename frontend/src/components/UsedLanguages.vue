<template>
  <BarChart :id="id" :series="series" />
</template>

<script lang="ts">
import { defineComponent } from "vue";

import { ChartSeriesData } from "./charts/BarChart.vue";
import BarChart from "./charts/BarChart.vue";

interface ResponseData {
  languages: {
    [lang: string]: {
      size: number;
      color: string;
    };
  };
}

export default defineComponent({
  name: "UsedLanguages",
  components: { BarChart },
  data: function () {
    return {
      id: "used-languages-chart",
      series: [] as ChartSeriesData[],
    };
  },
  methods: {
    init(json: ResponseData) {
      const langMap = json.languages;
      const keys = Object.keys(langMap);
      const seriesData = keys
        .map((key) => {
          return {
            name: key,
            y: langMap[key].size,
            color: langMap[key].color,
          };
        })
        .sort((a, b) => {
          if (a.y > b.y) {
            return -1;
          } else if (a.y === b.y) {
            return 0;
          } else {
            return 1;
          }
        });
      this.series = seriesData;
    },
  },
  mounted() {
    fetch("/api/langs")
      .then((response) => response.json())
      .then((json) => this.init(json));
  },
});
</script>
