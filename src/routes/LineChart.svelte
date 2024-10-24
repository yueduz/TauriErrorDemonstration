<script lang="ts">
  import Chart from "chart.js/auto";
  import { onMount } from "svelte";
  import { getContext } from "svelte";
  // 生成 x 轴标签（可选，根据需求定制）
  const labels = Array.from({ length: 6 }, (_, i) => i + 1);
  let chartDom: any;
  // 第一个参数必须与模板中的 ref 值匹配
  let points = [0, 7, 3, 10, 12, 9, 15];
  let upd:any = getContext("upd");

  onMount(() => {
    var myChart = new Chart(chartDom, {
      type: "line",
      data: {
        labels: labels,
        datasets: [
          {
            label: "# of Votes",
            data: points,
            borderWidth: 1,
          },
        ],
      },
      options: {
        animation: false,
        // 你的图表选项
        responsive: true, // 确保图表在窗口大小改变时调整大小
        maintainAspectRatio: false, // 允许图表不按比例缩放以填充容器
        scales: {
          y: {
            ticks: {
              display: false,
            },
          },
        },
      },
    });

    upd.fu = function (data: any) {
      console.log(data);
      
      myChart.data.datasets[0].data = data;
      // 重新渲染图表
      myChart.update();
    };
  });
</script>

<canvas bind:this={chartDom}> </canvas>

<style lang="css">
  canvas {
    height: 100vh;
    width: 100vw;
  }
</style>
