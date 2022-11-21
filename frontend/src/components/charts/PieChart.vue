<template>
  <div :id="id" />
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import Highcharts from "highcharts";

export interface ChartSeriesData {
  name: string;
  y: number;
}

export interface ChartData {
  id: string;
  title: string;
  series: ChartSeriesData[];
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
        plotBackgroundColor: undefined,
        plotBorderWidth: undefined,
        plotShadow: false,
        type: "pie",
        width: 400,
        height: 200,
      },

      legend: {
        layout: "vertical",
        backgroundColor: "transparent",
        align: "left",
        verticalAlign: "top",
        floating: true,
        x: 150,
        y: 50,
        itemStyle: {
          color: "#333333",
          cursor: "pointer",
          fontSize: "10px",
          fontWeight: "normal",
          textOverflow: "ellipsis",
        },
      },

      title: {
        text: chartData.title,
        style: { fontSize: "14px" },
      },

      tooltip: {
        pointFormat: "{point.y}({point.percentage:.1f}%)",
      },

      accessibility: {
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
            format: "{point.y}",
            distance: -25,
            filter: {
              property: "percentage",
              operator: ">",
              value: 4,
            },
          },
          showInLegend: true,
          size: 120,
          center: [50, 50],
        },
      },
      series: [
        {
          type: "pie",
          data: chartData.series,
        },
      ],
    },
    undefined
  );
};

export default defineComponent({
  name: "PieChart",
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
      title: this.title,
      series: this.series,
    });
  },
});
</script>
