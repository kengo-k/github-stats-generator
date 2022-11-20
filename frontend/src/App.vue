<template>
  <div id="charts"></div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import Highcharts from "highcharts";

export default defineComponent({
  name: "App",
  components: {},
  mounted() {
    Highcharts.chart("charts", {
      chart: {
        plotShadow: false,
        type: "pie",
      },
      title: {
        text: "Browser market shares in May, 2020",
      },
      tooltip: {
        pointFormat: "{series.name}: <b>{point.percentage:.1f}%</b>",
      },
      accessibility: {
        enabled: false,
        point: {
          valueSuffix: "%",
        },
      },
      plotOptions: {
        pie: {
          allowPointSelect: true,
          cursor: "pointer",
          dataLabels: {
            enabled: true,
            format: "<b>{point.name}</b>: {point.percentage:.1f} %",
          },
        },
      },
      series: [
        {
          type: "area",
          name: "Brands",
          data: [
            {
              name: "Chrome",
              y: 70.67,
              selected: true,
            },
            {
              name: "Edge",
              y: 14.77,
            },
            {
              name: "Firefox",
              y: 4.86,
            },
            {
              name: "Safari",
              y: 2.63,
            },
            {
              name: "Internet Explorer",
              y: 1.53,
            },
            {
              name: "Opera",
              y: 1.4,
            },
            {
              name: "Sogou Explorer",
              y: 0.84,
            },
            {
              name: "QQ",
              y: 0.51,
            },
            {
              name: "Other",
              y: 2.6,
            },
          ],
        },
      ],
    });
  },
});
</script>
