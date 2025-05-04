<template>
    <div class="controls-panel">
      <div class="inputs">
        <label>
          Start (mA):
          <input type="number" v-model.number="start" :disabled="isRunning" />
        </label>
        <label>
          Stop (mA):
          <input type="number" v-model.number="stop" :disabled="isRunning" />
        </label>
        <label>
          Step (mA):
          <input type="number" v-model.number="step" :disabled="isRunning" />
        </label>
      </div>
  
      <div class="actions">
        <button @click="startSweep" :disabled="!isValid || isRunning">
          Start Sweep
        </button>
        <span v-if="errorMsg" class="error">{{ errorMsg }}</span>
        <span v-if="isRunning">Progress: {{ progress }} / {{ totalSteps }}</span>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, computed } from 'vue'
  //import { invoke } from '@tauri-apps/api/tauri'
  import { core } from '@tauri-apps/api';
  const { invoke } = core;
  import { listen } from '@tauri-apps/api/event'
  import { useToast } from 'vue-toastification'
  const toast = useToast()

  
  interface SweepPoint {
    timestamp: string
    current_mA: number
    power_dBm: string
    module: number
  }

  const errorMsg = ref<string | null>(null)

const isValid = computed(() => {
  if (start.value >= stop.value) {
    errorMsg.value = 'Start must be less than Stop'
    return false
  }
  if (step.value <= 0) {
    errorMsg.value = 'Step must be greater than 0'
    return false
  }
  errorMsg.value = null
  return true
})

  
  // Props: which module (port) to use
  const module = 1
  
  // Emits
  const emit = defineEmits<{
    (e: 'data-point', point: SweepPoint): void
    (e: 'sweep-done', path: string): void
  }>()
  
  // Form values
  const start = ref(0)
  const stop = ref(100)
  const step = ref(5)
  
  const isRunning = ref(false)
  const progress = ref(0)
  const totalSteps = computed(() => {
    return Math.floor((stop.value - start.value) / step.value) + 1
  })
  
  async function startSweep() {
    isRunning.value = true
    progress.value = 0
  
    const unlisten = await listen<SweepPoint>('sweep-point', (event) => {
      emit('data-point', event.payload)
      progress.value++
    })
  
    try {
      const path = await invoke<string>('run_current_sweep', {
        module,
        startMa: start.value,
        stopMa: stop.value,
        stepMa: step.value
      })
      emit('sweep-done', path)
      toast.success('Sweep completed successfully!')
    } catch (err) {
      toast.error('Sweep failed: ' + err)
    } finally {
      await unlisten()
      isRunning.value = false
    }
  }
  </script>
  
  <style scoped>
  .error {
    color: red;
    font-weight: bold;
    margin-left: 12px;
  }

  .controls-panel {
    padding: 16px;
    border: 1px solid #ccc;
    border-radius: 8px;
    margin-bottom: 16px;
  }
  
  .inputs {
    display: flex;
    gap: 12px;
    margin-bottom: 12px;
  }
  
  label {
    display: flex;
    flex-direction: column;
    font-weight: bold;
  }
  
  input[type='number'] {
    padding: 6px;
    width: 100px;
  }
  
  .actions {
    display: flex;
    align-items: center;
    gap: 16px;
  }
  
  button {
    padding: 8px 16px;
    background-color: #0099ff;
    color: white;
    border: none;
    border-radius: 6px;
    cursor: pointer;
  }
  button:disabled {
    background-color: #aaa;
    cursor: not-allowed;
  }
  </style>
  