<template>
  <div id="languages-chart"></div>
</template>

<script>
  const initChart = (chartData) => {
    const totalSize = chartData.totalSize;
    const langMap = chartData.languages;
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
    Highcharts.chart("languages-chart", {
      chart: {
        plotBackgroundColor: null,
        plotBorderWidth: null,
        plotShadow: false,
        type: "pie",
      },
      title: {
        text: "Used languages in projects",
      },
      tooltip: {
        pointFormat: "{series.name}: <b>{point.percentage:.1f}%</b>",
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
            format: "<b>{point.name}</b>: {point.percentage:.1f} %",
          },
        },
      },
      series: [
        {
          name: "Brands",
          colorByPoint: true,
          data: seriesData,
        },
      ],
    });
  };

  export default {
    data: function() {
      return {
      }
    },
    mounted() {
      fetch("/api/langs")
        .then((response) => response.json())
        .then((data) => initChart(data));
    },
  }
</script>
