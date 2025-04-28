<template>
    <div class="temperature-control panel">
      <h3>TEC Temperature Control</h3>
      
      <div class="status-section">
        <div class="tec-status">
          <span>TEC Status:</span>
          <span class="status-indicator" :class="{ 'on': tecEnabled, 'off': !tecEnabled }">
            {{ tecEnabled ? 'ON' : 'OFF' }}
          </span>
          <button 
            @click="toggleTEC" 
            :disabled="!isConnected"
            class="toggle-button"
          >
            {{ tecEnabled ? 'Turn Off' : 'Turn On' }}
          </button>
        </div>
      </div>
      
      <div class="control-section">
        <parameter-slider 
          label="Temperature" 
          :value="temperature" 
          :min="10" 
          :max="40" 
          :step="0.1" 
          unit="°C"
          @update:value="updateTemperature"
        />
        
        <div class="temperature-readout">
          <label>Actual Temperature:</label>
          <span class="value">{{ actualTemperature }} °C</span>
        </div>
      </div>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref, onMounted, onUnmounted } from 'vue';
  import { core } from '@tauri-apps/api';
  const { invoke } = core;

  import ParameterSlider from '../common/ParameterSlider.vue';
  
  export default defineComponent({
    name: 'TemperatureControl',
    components: {
      ParameterSlider
    },
    props: {
      isConnected: {
        type: Boolean,
        required: true
      }
    },
    setup(props) {
      const tecEnabled = ref(false);
      const temperature = ref(25.0); // Default temperature in °C
      const actualTemperature = ref(0);
      
      const toggleTEC = async () => {
        if (!props.isConnected) return;
        
        try {
          if (tecEnabled.value) {
            // Turn off TEC
            await invoke('set_cld1015_tec_output', { enabled: false });
          } else {
            // Turn on TEC
            await invoke('set_cld1015_tec_output', { enabled: true });
          }
          // Update the state after toggling
          tecEnabled.value = !tecEnabled.value;
        } catch (error) {
          console.error('Failed to toggle TEC:', error);
        }
      };
      
      const updateTemperature = async (newValue: number) => {
        if (!props.isConnected) return;
        
        temperature.value = newValue;
        try {
          // Assuming we have a Tauri command to set temperature
          await invoke('set_cld1015_temperature', { temperatureC: newValue });
        } catch (error) {
          console.error('Failed to set temperature:', error);
        }
      };
      
      const refreshTECState = async () => {
        if (!props.isConnected) return;
        
        try {
          // Get TEC state
          const tecState = await invoke('get_cld1015_tec_state');
          tecEnabled.value = tecState as boolean;
          
          // Get actual temperature 
          if (tecEnabled.value) {
            const temp = await invoke('get_cld1015_temperature');
            actualTemperature.value = temp as number;
          }
        } catch (error) {
          console.error('Failed to refresh TEC state:', error);
        }
      };
      
      // Set up periodic refresh
      let refreshInterval: number | null = null;
      
      onMounted(() => {
        refreshInterval = window.setInterval(() => {
          if (props.isConnected) {
            refreshTECState();
          }
        }, 1000);
      });
      
      onUnmounted(() => {
        if (refreshInterval) {
          clearInterval(refreshInterval);
        }
      });
      
      return {
        tecEnabled,
        temperature,
        actualTemperature,
        toggleTEC,
        updateTemperature
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
  
  .status-section {
    margin-bottom: 15px;
  }
  
  .tec-status {
    display: flex;
    align-items: center;
    gap: 10px;
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
  
  .toggle-button:disabled {
    background-color: #90CAF9;
    cursor: not-allowed;
  }
  
  .control-section {
    margin-bottom: 20px;
    padding: 10px;
    border: 1px solid #e0e0e0;
    border-radius: 4px;
  }
  
  .temperature-readout {
    margin-top: 10px;
    display: flex;
    justify-content: space-between;
    padding: 8px;
    background-color: #e8f4f8;
    border-radius: 4px;
  }
  
  .value {
    font-weight: bold;
  }
  </style>
  