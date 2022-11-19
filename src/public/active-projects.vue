<template>
  <div id="chart"></div>
</template>

<script>
  const initChart = (data) => {
    data = [data]
    const repoMap = {};
    const dataMap = { keys: [], repos: repoMap };
    data.forEach((item) => {
      const since = item.since.substring(0, 7);
      if (!dataMap.keys.includes(since)) {
        dataMap.keys.push(since);
      }
      const repos = item.repos;
      for (const repo of repos) {
        const name = repo.name;
        if (!(name in repoMap)) {
          repoMap[name] = { name };
        }
        if (name in repoMap) {
          const data = repoMap[name];
          data[since] = repo.commit_count;
        }
      }
    });
    const repos = dataMap.repos;
    const series = Object.keys(repos).map((repoName) => {
      const repo = repos[repoName];
      const data = [];
      for (const key of dataMap.keys) {
        if (key in repo) {
          data.push(repo[key]);
        } else {
          data.push(0);
        }
      }
      return {
        name: repo.name,
        data,
      };
    });

    Highcharts.chart("chart", {
      chart: {
        type: "column",
      },
      legend: {
        enabled: false,
      },
      title: {
        text: "Commits by project in the last 3 months",
      },

      xAxis: {
        categories: dataMap.keys,
      },

      yAxis: {
        allowDecimals: false,
        min: 0,
        title: {
          text: "Commit",
        },
      },

      tooltip: {
        formatter: function () {
          return (
            "<b>" + this.x + "</b><br/>" + this.series.name + ": " + this.y
          );
        },
      },

      plotOptions: {
        column: {
          stacking: "normal",
        },
      },

      series,
    });
  };

  export default {
    data: function() {
      return {
      }
    },
    mounted() {
      fetch("/api/active_projects?since=2022-11-01&until=2022-11-06")
        .then((response) => response.json())
        .then((data) => initChart(data));
    }
  }
</script>
