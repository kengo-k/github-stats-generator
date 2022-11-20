<template>
  <div>
    <div :id="id" />
  </div>
</template>

<script>

  const initChart = ({ id, title, series }) => {

    Highcharts.chart(id, {

      chart: {
          plotBackgroundColor: null,
          plotBorderWidth: null,
          plotShadow: false,
          type: 'pie',
          width: 400,
          height: 200,
      },

      legend: {
        layout: 'vertical',
        backgroundColor: 'transparent',
        align: 'left',
        verticalAlign: 'top',
        floating: true,
        x: 150,
        y: 50,
        itemStyle: {"color": "#333333", "cursor": "pointer", "fontSize": "10px", "fontWeight": "normal", "textOverflow": "ellipsis"}
      },

      title: {
          text: title,
          style: { "fontSize": "14px" }
      },

      tooltip: {
          pointFormat: '{point.y}({point.percentage:.1f}%)'
      },

      accessibility: {
          point: {
              valueSuffix: '%'
          }
      },

      plotOptions: {
          pie: {
              allowPointSelect: true,
              cursor: 'pointer',
              dataLabels: {
                  enabled: true,
                  format: '{point.y}',
                  distance: -25,
                  filter: {
                      property: 'percentage',
                      operator: '>',
                      value: 4
                  }
              },
              showInLegend: true,
              size: 120,
              center: [50, 50],
          }
      },
      series: [{
          data: series
      }]
    })
  };

  export default {
    props: ["id", "title", "series"],
    updated() {
      console.log("[pie-chart] updated called");
      console.log("[pie-chart] id:", this.id);
      console.log("[pie-chart] title:", this.title);
      console.log("[pie-chart] series:", this.series);
      console.log("[pie-chart] categories:", this.categories);
      initChart({
        id: this.id,
        title: this.title,
        series: this.series
      });
    },
  }
</script>
