<template>
    <div class="main-view">
      <h1>Optical Lab Control Suite</h1>
  
      <DeviceControls @refresh-status="refreshStatus" />
      <DeviceStatus ref="deviceStatusRef" />

  
      <ControlsPanel
        @data-point="addPoint"
        @sweep-done="setCsvPath"
      />
  
      <LIPlot :data="liData" :csv-path="csvPath" />
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref } from 'vue'
  import DeviceStatus from '@/components/DeviceStatus.vue'
  import ControlsPanel from '@/components/ControlsPanel.vue'
  import LIPlot from '@/components/LIPlot.vue'
  import { SweepPoint } from '@/types/interfaces'
  
  interface DeviceStatusExposed {
  refreshStatus: () => void
  }


  const liData = ref<SweepPoint[]>([])
  const csvPath = ref<string | null>(null)
  
  const deviceStatusRef = ref<DeviceStatusExposed | null>(null)
  function refreshStatus() {
    deviceStatusRef.value?.refreshStatus()
  }

  
  function addPoint(point: SweepPoint) {
    liData.value.push(point)
  }
  
  function setCsvPath(path: string) {
    csvPath.value = path
  }
  </script>
  
  <style scoped>
  .main-view {
    max-width: 1000px;
    margin: 0 auto;
    padding: 24px;
    font-family: 'Segoe UI', Roboto, sans-serif;
  }
  
  h1 {
    text-align: center;
    margin-bottom: 24px;
  }
  </style>
  