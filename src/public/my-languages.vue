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
      console.log(seriesData);

      Highcharts.chart('languages-chart', {
        chart: {
            type: 'bar'
        },
        title: {
            align: 'left',
            text: 'Browser market shares. January, 2022'
        },
        subtitle: {
            align: 'left',
            text: 'Click the columns to view versions. Source: <a href="http://statcounter.com" target="_blank">statcounter.com</a>'
        },
        accessibility: {
            announceNewData: {
                enabled: true
            }
        },
        xAxis: {
            type: 'category'
        },
        yAxis: {
            title: {
                text: 'Total percent market share'
            }

        },
        legend: {
            enabled: false
        },
        plotOptions: {
            series: {
                borderWidth: 0,
                dataLabels: {
                    enabled: true,
                    format: '{point.y}'
                }
            }
        },

        tooltip: {
            headerFormat: '<span style="font-size:11px">{series.name}</span><br>',
            pointFormat: '<span style="color:{point.color}">{point.name}</span>: <b>{point.y:.2f}%</b> of total<br/>'
        },

        series: [
            {
                name: "Browsers",
                colorByPoint: true,
                data: seriesData
            }
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
