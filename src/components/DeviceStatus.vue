<template>
    <div class="device-status">
      <div class="status-item" :class="{ online: cldConnected }">
        CLD1015: {{ cldConnected ? 'Connected' : 'Disconnected' }}
      </div>
      <div class="status-item" :class="{ online: mpmConnected }">
        MPM210H: {{ mpmConnected ? 'Connected' : 'Disconnected' }}
      </div>
      <div class="status-item" :class="{ online: tecOn }">
        TEC: {{ tecOn ? 'ON' : 'OFF' }}
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted } from 'vue'
  //import { invoke } from '@tauri-apps/api/tauri'
  import { core } from '@tauri-apps/api';
  const { invoke } = core;
  
  const cldConnected = ref(false)
  const mpmConnected = ref(false)
  const tecOn = ref(false)
  
  async function refreshStatus() {
    cldConnected.value = await invoke('is_cld1015_connected')
    mpmConnected.value = await invoke('is_mpm210h_connected')
    if (cldConnected.value) {
      tecOn.value = await invoke('get_tec_state')
    } else {
      tecOn.value = false
    }
  }
  defineExpose({ refreshStatus })
  onMounted(refreshStatus)
  </script>
  
  <style scoped>
  .device-status {
    display: flex;
    gap: 16px;
    margin-bottom: 16px;
  }
  
  .status-item {
    padding: 8px 14px;
    border-radius: 6px;
    background-color: #ddd;
    color: black;
    font-weight: bold;
  }
  
  .status-item.online {
    background-color: #4caf50;
    color: white;
  }
  </style>
  