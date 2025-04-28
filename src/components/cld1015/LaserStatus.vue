<template>
    <div class="laser-status panel">
      <h3>Laser Status</h3>
      
      <div class="status-info">
        <div class="status-row">
          <span>Laser Output:</span>
          <span class="status-indicator" :class="{ 'on': laserEnabled, 'off': !laserEnabled }">
            {{ laserEnabled ? 'ON' : 'OFF' }}
          </span>
          <button 
            @click="toggleLaser" 
            :disabled="!isConnected || !tecEnabled"
            class="toggle-button"
            :class="{ 'warning': laserEnabled }"
          >
            {{ laserEnabled ? 'Turn Off' : 'Turn On' }}
          </button>
        </div>
        
        <div class="status-row">
          <span>TEC Status:</span>
          <span class="status-indicator" :class="{ 'on': tecEnabled, 'off': !tecEnabled }">
            {{ tecEnabled ? 'ON' : 'OFF' }}
          </span>
        </div>
        
        <div class="status-row">
          <span>Mode:</span>
          <select v-model="operationMode" @change="changeMode" :disabled="!isConnected || laserEnabled">
            <option value="current">Constant Current</option>
            <option value="power">Constant Power</option>
          </select>
        </div>
      </div>
      
      <div class="warning-message" v-if="!tecEnabled && isConnected">
        <p>⚠️ TEC must be enabled before turning on the laser</p>
      </div>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref, onMounted, onUnmounted } from 'vue';
  import { core } from '@tauri-apps/api';
  const { invoke } = core;

  
  export default defineComponent({
    name: 'LaserStatus',
    props: {
      isConnected: {
        type: Boolean,
        required: true
      },
      tecEnabled: {
        type: Boolean,
        required: true
      }
    },
    emits: ['mode-changed'],
    setup(props, { emit }) {
      const laserEnabled = ref(false);
      const operationMode = ref('current'); // 'current' or 'power'
      
      const toggleLaser = async () => {
        if (!props.isConnected || !props.tecEnabled) return;
        
        try {
          if (laserEnabled.value) {
            // Turn off laser
            await invoke('set_cld1015_laser_output', { enabled: false });
          } else {
            // Turn on laser
            await invoke('set_cld1015_laser_output', { enabled: true });
          }
          // Update the state after toggling
          laserEnabled.value = !laserEnabled.value;
        } catch (error) {
          console.error('Failed to toggle laser:', error);
        }
      };
      
      const changeMode = async () => {
        if (!props.isConnected || laserEnabled.value) return;
        
        try {
          // Assuming we have a command to set the mode
          await invoke('set_cld1015_mode', { mode: operationMode.value });
          emit('mode-changed', operationMode.value);
        } catch (error) {
          console.error('Failed to change operation mode:', error);
        }
      };
      
      const refreshLaserState = async () => {
        if (!props.isConnected) return;
        
        try {
          // Get laser output state
          const laserState = await invoke('get_cld1015_laser_output');
          laserEnabled.value = laserState as boolean;
        } catch (error) {
          console.error('Failed to refresh laser state:', error);
        }
      };
      
      // Set up periodic refresh
      let refreshInterval: number | null = null;
      
      onMounted(() => {
        refreshInterval = window.setInterval(() => {
          if (props.isConnected) {
            refreshLaserState();
          }
        }, 1000);
      });
      
      onUnmounted(() => {
        if (refreshInterval) {
          clearInterval(refreshInterval);
        }
      });
      
      return {
        laserEnabled,
        operationMode,
        toggleLaser,
        changeMode
      };
    }
  });
  </script>
  
  <style scoped>
  .panel {
    background-color: #f9f9f9;
    border-radius: 8px;
    padding: 15px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    margin-top: 20px;
  }
  
  .status-info {
    padding: 10px;
    border: 1px solid #e0e0e0;
    border-radius: 4px;
  }
  
  .status-row {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 10px;
  }
  
  .status-row span:first-child {
    width: 100px;
  }
  
  .status-indicator {
    font-weight: bold;
    padding: 3px 8px;
    border-radius: 4px;
  }
  
  .on {
    background-color: #4CAF50;
    color: white;
  }
  
  .off {
    background-color: #f44336;
    color: white;
  }
  
  .toggle-button {
    padding: 5px 10px;
    background-color: #2196F3;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  
  .toggle-button.warning {
    background-color: #f44336;
  }
  
  .toggle-button:disabled {
    background-color: #90CAF9;
    cursor: not-allowed;
  }
  
  .warning-message {
    margin-top: 10px;
    padding: 10px;
    background-color: #fff3e0;
    border-left: 4px solid #ff9800;
    border-radius: 4px;
  }
  
  select {
    padding: 5px;
    border-radius: 4px;
    border: 1px solid #ccc;
  }
  
  select:disabled {
    background-color: #f5f5f5;
    cursor: not-allowed;
  }
  </style>
  