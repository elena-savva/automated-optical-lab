<template>
    <div class="li-plot-container">
      <LineChart :chart-data="chartData" :chart-options="chartOptions" />
    </div>
  </template>
  
  <script setup lang="ts">
  import { computed, defineProps } from 'vue'
  import { LineChart } from 'vue-chart-3'
  import { Chart, registerables } from 'chart.js'
  Chart.register(...registerables)
  
  interface SweepPoint {
    current_mA: number
    power_dBm: string
  }
  
  const props = defineProps<{
    data: SweepPoint[]
  }>()
  
  // Reactive Chart.js format
  const chartData = computed(() => ({
    datasets: [
      {
        label: 'L-I Curve',
        data: props.data.map(p => ({
          x: p.current_mA,
          y: parseFloat(p.power_dBm)
        })),
        borderColor: 'blue',
        backgroundColor: 'lightblue',
        tension: 0.2,
        fill: false,
        borderWidth: 2
      }
    ]
  }))
  
  const chartOptions = {
    responsive: true,
    animation: false,
    scales: {
      x: {
        title: {
          display: true,
          text: 'Current (mA)'
        }
      },
      y: {
        title: {
          display: true,
          text: 'Power (dBm)'
        }
      }
    }
  }
  </script>
  
  <style scoped>
  .li-plot-container {
    width: 100%;
    height: 400px;
  }
  </style>
  