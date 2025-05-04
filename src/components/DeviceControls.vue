<template>
  <div class="device-controls">
    <button @click="connectCLD" :disabled="loading">Connect CLD1015</button>
    <button @click="connectMPM" :disabled="loading">Connect MPM210H</button>
    <button @click="$emit('refresh-status')">Refresh Device Status</button>
  </div>
</template>
  
  <script setup lang="ts">
  import { ref } from 'vue'
  //import { invoke } from '@tauri-apps/api/tauri'
  import { core } from '@tauri-apps/api';
  const { invoke } = core;
  import { useToast, POSITION } from 'vue-toastification'
  
  const toast = useToast()
  const loading = ref(false)
  
  const emit = defineEmits<{
    (e: 'refresh-status'): void
  }>()
  
  async function connectCLD() {
    loading.value = true
    try {
      await invoke('connect_cld1015')
      toast.success('CLD1015 connected', { position: POSITION.TOP_RIGHT })
      emit('refresh-status')
    } catch (e) {
      toast.error('Failed to connect CLD1015: ' + e, { position: POSITION.TOP_RIGHT })
    } finally {
      loading.value = false
    }
  }
  
  async function connectMPM() {
    loading.value = true
    try {
      await invoke('connect_mpm210h')
      toast.success('MPM210H connected', { position: POSITION.TOP_RIGHT })
      emit('refresh-status')
    } catch (e) {
      toast.error('Failed to connect MPM210H: ' + e, { position: POSITION.TOP_RIGHT })
    } finally {
      loading.value = false
    }
  }
  </script>
  
  <style scoped>
  .device-controls {
    display: flex;
    gap: 12px;
    margin-bottom: 16px;
  }
  
  button {
    padding: 6px 16px;
    background-color: #0080ff;
    color: white;
    border: none;
    border-radius: 6px;
    cursor: pointer;
  }
  button:disabled {
    background-color: #999;
    cursor: not-allowed;
  }
  </style>
  