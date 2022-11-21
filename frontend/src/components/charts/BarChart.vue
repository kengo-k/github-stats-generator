<template>
  <div :id="id" />
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import Highcharts from "highcharts";

export interface ChartSeriesData {
  name: string;
  y: number;
  color: string;
}

const initChart = (chartData: { id: string; series: ChartSeriesData[] }) => {
  Highcharts.chart(
    chartData.id,
    {
      chart: {
        type: "bar",
      },
      title: {
        align: "left",
        text: "Browser market shares. January, 2022",
      },
      subtitle: {
        align: "left",
        text: 'Click the columns to view versions. Source: <a href="http://statcounter.com" target="_blank">statcounter.com</a>',
      },
      accessibility: {
        announceNewData: {
          enabled: true,
        },
      },
      xAxis: {
        type: "category",
      },
      yAxis: {
        title: {
          text: "Total percent market share",
        },
      },
      legend: {
        enabled: false,
      },
      plotOptions: {
        series: {
          borderWidth: 0,
          dataLabels: {
            enabled: true,
            format: "{point.y}",
          },
        },
      },

      tooltip: {
        headerFormat: '<span style="font-size:11px">{series.name}</span><br>',
        pointFormat:
          '<span style="color:{point.color}">{point.name}</span>: <b>{point.y:.2f}%</b> of total<br/>',
      },

      series: [
        {
          type: "bar",
          name: "Languages",
          colorByPoint: true,
          data: chartData.series,
        },
      ],
    },
    undefined
  );
};

export default defineComponent({
  name: "BarChart",
  props: {
    id: {
      type: String,
      required: true,
    },
    series: {
      type: Array as PropType<ChartSeriesData[]>,
      required: true,
    },
  },
  updated() {
    initChart({
      id: this.id,
      series: this.series,
    });
  },
});
</script>
