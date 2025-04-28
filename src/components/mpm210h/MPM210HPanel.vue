<template>
    <div class="mpm210h-panel">
      <h2>Santec MPM-210H Power Meter</h2>
      
      <connection-status 
        deviceName="MPM-210H" 
        :isConnected="isConnected" 
        @connect="connectDevice" 
        @disconnect="disconnectDevice"
      />
      
      <div v-if="isConnected || hasBeenConnected" class="controls-container">
        <div class="panel wavelength-panel">
          <h3>Wavelength Control</h3>
          <div class="control-row">
            <label>Wavelength (nm):</label>
            <input 
              type="number" 
              v-model.number="wavelength" 
              :disabled="!isConnected"
              @change="setWavelength"
              min="800"
              max="1700"
              step="0.1"
            />
            <button 
              @click="setWavelength" 
              :disabled="!isConnected"
              class="control-button"
            >
              Set
            </button>
          </div>
          
          <div class="current-value">
            <span>Current Wavelength:</span>
            <span class="value">{{ currentWavelength }} nm</span>
          </div>
        </div>
        
        <div class="panel power-panel">
          <h3>Power Measurement</h3>
          
          <div class="module-selection">
            <label>Select Module:</label>
            <select v-model="selectedModule" @change="readPower" :disabled="!isConnected">
              <option v-for="module in availableModules" :key="module" :value="module">
                Module {{ module }}
              </option>
            </select>
            
            <button 
              @click="readPower" 
              :disabled="!isConnected"
              class="control-button"
            >
              Read Power
            </button>
          </div>
          
          <div class="power-readout">
            <div v-for="(value, index) in powerValues" :key="index" class="power-value">
              <span>Channel {{ index + 1 }}:</span>
              <span class="value">{{ value }} dBm</span>
            </div>
          </div>
          
          <div class="continuous-reading">
            <label>
              <input type="checkbox" v-model="continuousReading" :disabled="!isConnected" />
              Continuous Reading
            </label>
          </div>
        </div>
      </div>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref, watch, onMounted, onUnmounted } from 'vue';
  import { core } from '@tauri-apps/api';
  const { invoke } = core;

  import ConnectionStatus from '../common/ConnectionStatus.vue';
  
  export default defineComponent({
    name: 'MPM210HPanel',
    components: {
      ConnectionStatus
    },
    setup() {
      const isConnected = ref(false);
      const hasBeenConnected = ref(false);
      const wavelength = ref(1550);
      const currentWavelength = ref('--');
      const selectedModule = ref(0);
      const availableModules = ref<number[]>([0]);
      const powerValues = ref<string[]>([]);
      const continuousReading = ref(false);
      
      let readingInterval: number | null = null;
      
      const connectDevice = async () => {
        try {
          const response = await invoke('connect_mpm210h');
          console.log('Connection response:', response);
          isConnected.value = true;
          hasBeenConnected.value = true;
          
          // Get initial wavelength
          await getWavelength();
          
          // Get recognized modules
          await getModules();
        } catch (error) {
          console.error('Failed to connect to MPM-210H:', error);
        }
      };
      
      const disconnectDevice = async () => {
        try {
          // In a real app, you'd have a disconnect command
          // await invoke('disconnect_mpm210h');
          isConnected.value = false;
          stopContinuousReading();
        } catch (error) {
          console.error('Failed to disconnect from MPM-210H:', error);
        }
      };
      
      const getWavelength = async () => {
        if (!isConnected.value) return;
        
        try {
          const result = await invoke('get_mpm210h_wavelength');
          currentWavelength.value = result as string;
          // Try to parse and update the input field
          const numValue = parseFloat(currentWavelength.value);
          if (!isNaN(numValue)) {
            wavelength.value = numValue;
          }
        } catch (error) {
          console.error('Failed to get wavelength:', error);
        }
      };
      
      const setWavelength = async () => {
        if (!isConnected.value) return;
        
        try {
          await invoke('set_mpm210h_wavelength', { wavelength: wavelength.value });
          await getWavelength(); // Refresh to confirm
        } catch (error) {
          console.error('Failed to set wavelength:', error);
        }
      };
      
      const getModules = async () => {
        if (!isConnected.value) return;
        
        try {
          const result = await invoke('get_mpm210h_modules');
          const modulesStr = result as string;
          console.log('Modules string:', modulesStr);
          
          // Parse modules string - assuming format like "0,1,2"
          const modules = modulesStr.split(',')
            .map(m => parseInt(m.trim()))
            .filter(m => !isNaN(m));
          
          if (modules.length > 0) {
            availableModules.value = modules;
            selectedModule.value = modules[0];
          }
        } catch (error) {
          console.error('Failed to get modules:', error);
        }
      };
      
      const readPower = async () => {
        if (!isConnected.value) return;
        
        try {
          const result = await invoke('read_mpm210h_power', { 
            module: selectedModule.value 
          });
          const powerStr = result as string;
          console.log('Power reading:', powerStr);
          
          // Parse power string - assuming format like "-10.5,-11.2,-9.8,-10.1"
          powerValues.value = powerStr.split(',').map(p => p.trim());
        } catch (error) {
          console.error('Failed to read power:', error);
        }
      };
      
      const startContinuousReading = () => {
        if (readingInterval !== null) return;
        
        readingInterval = window.setInterval(() => {
          readPower();
        }, 1000); // Read every second
      };
      
      const stopContinuousReading = () => {
        if (readingInterval !== null) {
          clearInterval(readingInterval);
          readingInterval = null;
        }
      };
      
      // Watch for changes to continuousReading
      watch(continuousReading, (newValue) => {
        if (newValue) {
          startContinuousReading();
        } else {
          stopContinuousReading();
        }
      });
      
      // Clean up on unmount
      onUnmounted(() => {
        stopContinuousReading();
      });
      
      onMounted(async () => {
        // Check if device is already connected
        try {
          const connected = await invoke('is_mpm210h_connected');
          isConnected.value = connected as boolean;
          if (isConnected.value) {
            hasBeenConnected.value = true;
            await getWavelength();
            await getModules();
          }
        } catch (error) {
          console.error('Failed to check MPM-210H connection status:', error);
        }
      });
      
      return {
        isConnected,
        hasBeenConnected,
        wavelength,
        currentWavelength,
        selectedModule,
        availableModules,
        powerValues,
        continuousReading,
        connectDevice,
        disconnectDevice,
        setWavelength,
        readPower
      };
    }
  });
  </script>
  
  <style scoped>
  .mpm210h-panel {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
  }
  
  .controls-container {
    margin-top: 20px;
  }
  
  .panel {
    background-color: #f9f9f9;
    border-radius: 8px;
    padding: 15px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    margin-bottom: 20px;
  }
  
  h2 {
    margin-bottom: 20px;
    color: #2c3e50;
  }
  
  h3 {
    margin-bottom: 15px;
    color: #2c3e50;
  }
  
  .control-row {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 15px;
  }
  
  .control-row label {
    width: 130px;
  }
  
  .control-row input {
    padding: 5px;
    border-radius: 4px;
    border: 1px solid #ccc;
    width: 100px;
  }
  
  .control-button {
    padding: 5px 10px;
    background-color: #2196F3;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  
  .control-button:disabled {
    background-color: #90CAF9;
    cursor: not-allowed;
  }
  
  .current-value {
    display: flex;
    justify-content: space-between;
    padding: 10px;
    background-color: #e8f4f8;
    border-radius: 4px;
  }
  
  .module-selection {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 15px;
  }
  
  .module-selection label {
    width: 110px;
  }
  
  .module-selection select {
    padding: 5px;
    border-radius: 4px;
    border: 1px solid #ccc;
  }
  
  .power-readout {
    padding: 10px;
    background-color: #e8f4f8;
    border-radius: 4px;
    margin-bottom: 15px;
  }
  
  .power-value {
    display: flex;
    justify-content: space-between;
    margin-bottom: 5px;
  }
  
  .value {
    font-weight: bold;
  }
  
  .continuous-reading {
    display: flex;
    justify-content: flex-end;
  }
  </style>
  