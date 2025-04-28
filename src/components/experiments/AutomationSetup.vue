<template>
    <div class="automation-setup">
      <h2>Experiment Automation</h2>
      
      <div class="panel">
        <h3>Combined Experiment Setup</h3>
        
        <div class="status-section">
          <div class="device-status">
            <span>CLD1015:</span>
            <span class="status-indicator" :class="{ 'on': cld1015Connected, 'off': !cld1015Connected }">
              {{ cld1015Connected ? 'Connected' : 'Disconnected' }}
            </span>
          </div>
          
          <div class="device-status">
            <span>MPM-210H:</span>
            <span class="status-indicator" :class="{ 'on': mpm210hConnected, 'off': !mpm210hConnected }">
              {{ mpm210hConnected ? 'Connected' : 'Disconnected' }}
            </span>
          </div>
        </div>
        
        <div class="experiment-config">
          <h4>Experiment Configuration</h4>
          
          <div class="config-section">
            <div class="input-group">
              <label>Start Current (mA):</label>
              <input type="number" v-model.number="startCurrent" min="0" max="150" step="1" />
            </div>
            
            <div class="input-group">
              <label>End Current (mA):</label>
              <input type="number" v-model.number="endCurrent" min="0" max="150" step="1" />
            </div>
            
            <div class="input-group">
              <label>Current Step (mA):</label>
              <input type="number" v-model.number="currentStep" min="0.1" max="10" step="0.1" />
            </div>
            
            <div class="input-group">
              <label>Delay Between Points (ms):</label>
              <input type="number" v-model.number="delayBetweenPoints" min="100" max="10000" step="100" />
            </div>
            
            <div class="input-group">
              <label>Number of Readings per Point:</label>
              <input type="number" v-model.number="readingsPerPoint" min="1" max="10" step="1" />
            </div>
            
            <div class="input-group">
              <label>Module to Read:</label>
              <select v-model.number="moduleToRead">
                <option v-for="module in availableModules" :key="module" :value="module">
                  Module {{ module }}
                </option>
              </select>
            </div>
          </div>
        </div>
        
        <div class="buttons-section">
          <button 
            @click="startExperiment" 
            :disabled="!canStartExperiment || isRunning"
            class="primary-button"
          >
            Start Experiment
          </button>
          
          <button 
            @click="stopExperiment" 
            :disabled="!isRunning"
            class="secondary-button"
          >
            Stop Experiment
          </button>
        </div>
        
        <div class="progress-section" v-if="isRunning">
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: progressPercentage + '%' }"></div>
          </div>
          <div class="progress-text">
            {{ currentPoint }} / {{ totalPoints }} points ({{ progressPercentage }}%)
          </div>
        </div>
      </div>
      
      <div class="panel results-panel" v-if="experimentData.length > 0">
        <h3>Experiment Results</h3>
        
        <div class="results-table-container">
          <table class="results-table">
            <thead>
              <tr>
                <th>Current (mA)</th>
                <th v-for="(_, index) in channelCount" :key="index">
                  Power Ch {{ index + 1 }} (dBm)
                </th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(point, index) in experimentData" :key="index">
                <td>{{ point.current.toFixed(1) }}</td>
                <td v-for="(power, chIndex) in point.powers" :key="chIndex">
                  {{ power }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
        
        <div class="export-section">
          <button @click="exportData" class="export-button">
            Export Data (CSV)
          </button>
        </div>
      </div>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref, computed, onMounted, onUnmounted } from 'vue';
  import { core } from '@tauri-apps/api';
  const { invoke } = core;

  
  interface DataPoint {
    current: number;
    powers: string[];
  }
  
  export default defineComponent({
    name: 'AutomationSetup',
    setup() {
      const cld1015Connected = ref(false);
      const mpm210hConnected = ref(false);
      const isRunning = ref(false);
      
      // Experiment configuration
      const startCurrent = ref(10);
      const endCurrent = ref(100);
      const currentStep = ref(1);
      const delayBetweenPoints = ref(500);
      const readingsPerPoint = ref(3);
      const moduleToRead = ref(0);
      const availableModules = ref<number[]>([0]);
      
      // Progress tracking
      const currentPoint = ref(0);
      const totalPoints = computed(() => {
        return Math.floor((endCurrent.value - startCurrent.value) / currentStep.value) + 1;
      });
      const progressPercentage = computed(() => {
        if (totalPoints.value === 0) return 0;
        return Math.round((currentPoint.value / totalPoints.value) * 100);
      });
      
      // Results
      const experimentData = ref<DataPoint[]>([]);
      const channelCount = ref(4); // Default to 4 channels
      
      // Computed properties
      const canStartExperiment = computed(() => {
        return cld1015Connected.value && mpm210hConnected.value &&
               startCurrent.value < endCurrent.value &&
               currentStep.value > 0;
      });
      
      // Check device connections
      const checkConnections = async () => {
        try {
          cld1015Connected.value = await invoke('is_cld1015_connected') as boolean;
          mpm210hConnected.value = await invoke('is_mpm210h_connected') as boolean;
          
          if (mpm210hConnected.value) {
            // Get available modules
            const modulesStr = await invoke('get_mpm210h_modules') as string;
            const modules = modulesStr.split(',')
              .map(m => parseInt(m.trim()))
              .filter(m => !isNaN(m));
            
            if (modules.length > 0) {
              availableModules.value = modules;
              moduleToRead.value = modules[0];
            }
          }
        } catch (error) {
          console.error('Failed to check connections:', error);
        }
      };
      
      // Run the experiment
      const startExperiment = async () => {
        if (!canStartExperiment.value || isRunning.value) return;
        
        isRunning.value = true;
        currentPoint.value = 0;
        experimentData.value = [];
        
        try {
          // Make sure laser is on with TEC enabled
          const tecState = await invoke('get_cld1015_tec_state') as boolean;
          if (!tecState) {
            await invoke('set_cld1015_tec_output', { enabled: true });
            // Wait for TEC to stabilize
            await new Promise(resolve => setTimeout(resolve, 2000));
          }
          
          // Turn on laser if not already on
          const laserState = await invoke('get_cld1015_laser_output') as boolean;
          if (!laserState) {
            await invoke('set_cld1015_laser_output', { enabled: true });
            // Wait for laser to stabilize
            await new Promise(resolve => setTimeout(resolve, 1000));
          }
          
          // Run through each current step
          for (
            let current = startCurrent.value;
            current <= endCurrent.value && isRunning.value;
            current += currentStep.value
          ) {
            // Set current
            await invoke('set_cld1015_current', { currentMa: current });
            
            // Wait for stabilization
            await new Promise(resolve => setTimeout(resolve, delayBetweenPoints.value));
            
            // Collect readings
            let powerReadings: string[][] = [];
            for (let i = 0; i < readingsPerPoint.value && isRunning.value; i++) {
              const powerStr = await invoke('read_mpm210h_power', { 
                module: moduleToRead.value 
              }) as string;
              
              const powers = powerStr.split(',').map(p => p.trim());
              powerReadings.push(powers);
              
              // Update channelCount based on the first reading
              if (i === 0) {
                channelCount.value = powers.length;
              }
              
              // Small delay between readings
              if (i < readingsPerPoint.value - 1) {
                await new Promise(resolve => setTimeout(resolve, 100));
              }
            }
            
            // Average the readings (if experiment wasn't stopped)
            if (isRunning.value) {
              // For simplicity, we're just using the last reading
              // In a real app, you'd average all readings
              const lastReading = powerReadings[powerReadings.length - 1];
              
              experimentData.value.push({
                current,
                powers: lastReading
              });
              
              currentPoint.value++;
            }
          }
        } catch (error) {
          console.error('Experiment error:', error);
        } finally {
          isRunning.value = false;
        }
      };
      
      const stopExperiment = () => {
        isRunning.value = false;
      };
      
      const exportData = () => {
        if (experimentData.value.length === 0) return;
        
        // Create CSV content
        let csv = 'Current (mA)';
        for (let i = 0; i < channelCount.value; i++) {
          csv += `,Power Ch ${i + 1} (dBm)`;
        }
        csv += '\n';
        
        experimentData.value.forEach(point => {
          csv += point.current.toFixed(1);
          point.powers.forEach(power => {
            csv += `,${power}`;
          });
          csv += '\n';
        });
        
        // In a real app, you'd use Tauri's dialog and fs APIs to save the file
        console.log('CSV data:', csv);
        
        // For this example, just create a download
        const blob = new Blob([csv], { type: 'text/csv' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.setAttribute('href', url);
        a.setAttribute('download', `experiment_data_${new Date().toISOString().slice(0, 19).replace(/:/g, '-')}.csv`);
        a.click();
      };
      
      onMounted(() => {
        checkConnections();
        
        // Set up a periodic connection check
        const checkInterval = setInterval(checkConnections, 5000);
        
        // Clean up on component unmount
        onUnmounted(() => {
          clearInterval(checkInterval);
          if (isRunning.value) {
            stopExperiment();
          }
        });
      });
      
      return {
        cld1015Connected,
        mpm210hConnected,
        isRunning,
        startCurrent,
        endCurrent,
        currentStep,
        delayBetweenPoints,
        readingsPerPoint,
        moduleToRead,
        availableModules,
        currentPoint,
        totalPoints,
        progressPercentage,
        experimentData,
        channelCount,
        canStartExperiment,
        startExperiment,
        stopExperiment,
        exportData
      };
    }
  });
  </script>
  
  <style scoped>
  .automation-setup {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
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
  
  h3, h4 {
    margin-bottom: 15px;
    color: #2c3e50;
  }
  
  .status-section {
    display: flex;
    gap: 20px;
    margin-bottom: 20px;
  }
  
  .device-status {
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
  
  .experiment-config {
    margin-bottom: 20px;
  }
  
  .config-section {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 15px;
    padding: 15px;
    border: 1px solid #e0e0e0;
    border-radius: 4px;
  }
  
  .input-group {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }
  
  .input-group label {
    font-size: 14px;
    color: #666;
  }
  
  .input-group input, .input-group select {
    padding: 8px;
    border-radius: 4px;
    border: 1px solid #ccc;
  }
  
  .buttons-section {
    display: flex;
    gap: 10px;
    margin-bottom: 15px;
  }
  
  .primary-button, .secondary-button, .export-button {
    padding: 10px 16px;
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
  
  .export-button {
    background-color: #2196F3;
    color: white;
  }
  
  .progress-section {
    margin-top: 15px;
  }
  
  .progress-bar {
    height: 10px;
    background-color: #e0e0e0;
    border-radius: 5px;
    overflow: hidden;
    margin-bottom: 5px;
  }
  
  .progress-fill {
    height: 100%;
    background-color: #4CAF50;
    transition: width 0.3s ease;
  }
  
  .progress-text {
    text-align: center;
    font-size: 14px;
    color: #666;
  }
  
  .results-panel {
    margin-top: 30px;
  }
  
  .results-table-container {
    max-height: 300px;
    overflow-y: auto;
    margin-bottom: 15px;
  }
  
  .results-table {
    width: 100%;
    border-collapse: collapse;
  }
  
  .results-table th, .results-table td {
    padding: 8px;
    text-align: center;
    border-bottom: 1px solid #e0e0e0;
  }
  
  .results-table th {
    background-color: #f5f5f5;
    position: sticky;
    top: 0;
  }
  
  .export-section {
    display: flex;
    justify-content: flex-end;
  }
  </style>
  