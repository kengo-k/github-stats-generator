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
  size: number;
  dataLabels?: {
    format: string;
  };
}

const initChart = (chartData: {
  id: string;
  title: string;
  series: ChartSeriesData[];
}) => {
  Highcharts.chart(
    chartData.id,
    {
      chart: {
        type: "bar",
      },
      title: {
        align: "left",
        text: chartData.title,
      },
      accessibility: {
        enabled: false,
        announceNewData: {
          enabled: true,
        },
      },
      xAxis: {
        type: "category",
      },
      yAxis: {
        min: 0,
        max: 100,
        title: {
          text: "Percentage",
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
            format: "{point.y}K",
          },
        },
      },

      tooltip: {
        headerFormat: "",
        pointFormat:
          '<span style="color:{point.color}">{point.name}</span>: <br/>Size: <b>{point.size}K</b><br/>Ratio: <b>{point.y:.1f}%</b>',
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
    title: {
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
      title: this.title,
    });
  },
});
</script>
