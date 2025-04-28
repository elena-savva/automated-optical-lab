<template>
    <div class="current-control panel">
      <h3>Laser Current Control</h3>
      
      <div class="control-section">
        <parameter-slider 
          label="Current" 
          :value="current" 
          :min="0" 
          :max="maxCurrent" 
          :step="0.1" 
          unit="mA"
          @update:value="updateCurrent"
        />
        
        <div class="current-readout">
          <label>Actual Current:</label>
          <span class="value">{{ actualCurrent }} mA</span>
        </div>
      </div>
      
      <div class="control-section">
        <h4>Automation Settings</h4>
        <div class="input-group">
          <label>Min Current:</label>
          <input type="number" v-model.number="minCurrent" :max="maxCurrent" :step="0.1" /> mA
        </div>
        
        <div class="input-group">
          <label>Max Current:</label>
          <input type="number" v-model.number="maxCurrent" :min="minCurrent" :step="0.1" /> mA
        </div>
        
        <div class="input-group">
          <label>Step Size:</label>
          <input type="number" v-model.number="stepSize" :min="0.1" :step="0.1" /> mA
        </div>
        
        <div class="input-group">
          <label>Interval:</label>
          <input type="number" v-model.number="interval" :min="100" :step="100" /> ms
        </div>
      </div>
      
      <div class="button-group">
        <button 
          @click="startSweep" 
          :disabled="isSweeping || !isConnected" 
          class="primary-button"
        >
          Start Sweep
        </button>
        
        <button 
          @click="stopSweep" 
          :disabled="!isSweeping" 
          class="secondary-button"
        >
          Stop Sweep
        </button>
      </div>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref, onUnmounted } from 'vue';
  import { core } from '@tauri-apps/api';
  const { invoke } = core;

  import ParameterSlider from '../common/ParameterSlider.vue';
  
  export default defineComponent({
    name: 'CurrentControl',
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
      const current = ref(0);
      const actualCurrent = ref(0);
      const minCurrent = ref(0);
      const maxCurrent = ref(150); // Default max current in mA
      const stepSize = ref(1);
      const interval = ref(1000);
      const isSweeping = ref(false);
      
      let sweepInterval: number | null = null;
      
      const updateCurrent = async (newValue: number) => {
        if (!props.isConnected) return;
        
        current.value = newValue;
        try {
          await invoke('set_cld1015_current', { currentMa: newValue });
          // After setting, get the actual current
          await refreshActualCurrent();
        } catch (error) {
          console.error('Failed to set current:', error);
        }
      };
      
      const refreshActualCurrent = async () => {
        if (!props.isConnected) return;
        
        try {
          const result = await invoke('get_cld1015_current');
          actualCurrent.value = result as number;
        } catch (error) {
          console.error('Failed to get current:', error);
        }
      };
      
      const startSweep = () => {
        if (isSweeping.value || !props.isConnected) return;
        
        isSweeping.value = true;
        current.value = minCurrent.value;
        updateCurrent(current.value);
        
        sweepInterval = window.setInterval(async () => {
          let nextCurrent = current.value + stepSize.value;
          
          if (nextCurrent > maxCurrent.value) {
            if (!props.isConnected) {
              stopSweep();
              return;
            }
            nextCurrent = minCurrent.value; // Reset to min for continuous sweep
          }
          
          await updateCurrent(nextCurrent);
        }, interval.value);
      };
      
      const stopSweep = () => {
        if (sweepInterval) {
          clearInterval(sweepInterval);
          sweepInterval = null;
        }
        isSweeping.value = false;
      };
      
      // Clean up interval on component unmount
      onUnmounted(() => {
        stopSweep();
      });
      
      // Refresh actual current periodically when connected
      const startRefreshTimer = () => {
        const refreshTimer = setInterval(() => {
          if (props.isConnected) {
            refreshActualCurrent();
          }
        }, 1000); // Refresh every second
        
        onUnmounted(() => {
          clearInterval(refreshTimer);
        });
      };
      
      startRefreshTimer();
      
      return {
        current,
        actualCurrent,
        minCurrent,
        maxCurrent,
        stepSize,
        interval,
        isSweeping,
        updateCurrent,
        startSweep,
        stopSweep
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
  }
  
  .control-section {
    margin-bottom: 20px;
    padding: 10px;
    border: 1px solid #e0e0e0;
    border-radius: 4px;
  }
  
  .current-readout {
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
  
  .input-group {
    display: flex;
    align-items: center;
    margin-bottom: 8px;
  }
  
  .input-group label {
    width: 100px;
    flex-shrink: 0;
  }
  
  .input-group input {
    width: 80px;
    padding: 4px;
    margin-right: 5px;
    border: 1px solid #ccc;
    border-radius: 4px;
  }
  
  .button-group {
    display: flex;
    gap: 10px;
  }
  
  .primary-button, .secondary-button {
    padding: 8px 16px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: bold;
  }
  
  .primary-button {
    background-color: #4CAF50;
    color: white;
  }
  
  .primary-button:disabled {
    background-color: #a5d6a7;
    cursor: not-allowed;
  }
  
  .secondary-button {
    background-color: #f44336;
    color: white;
  }
  
  .secondary-button:disabled {
    background-color: #ef9a9a;
    cursor: not-allowed;
  }
  </style>
  