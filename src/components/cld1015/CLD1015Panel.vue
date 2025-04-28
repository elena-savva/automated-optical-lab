<template>
    <div class="cld1015-panel">
      <h2>CLD1015 Laser Controller</h2>
      
      <connection-status 
        deviceName="CLD1015" 
        :isConnected="isConnected" 
        @connect="connectDevice" 
        @disconnect="disconnectDevice"
      />
      
      <div v-if="isConnected || hasBeenConnected" class="controls-container">
        <laser-status 
          :isConnected="isConnected" 
          :tecEnabled="tecEnabled" 
          @mode-changed="onModeChanged"
        />
        
        <temperature-control 
          :isConnected="isConnected"
          @tec-state-changed="onTecStateChanged"
        />
        
        <current-control :isConnected="isConnected" />
      </div>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref, onMounted } from 'vue';
  import { core } from '@tauri-apps/api';
  const { invoke } = core;
  import ConnectionStatus from '../common/ConnectionStatus.vue';
  import LaserStatus from './LaserStatus.vue';
  import TemperatureControl from './TemperatureControl.vue';
  import CurrentControl from './CurrentControl.vue';
  
  export default defineComponent({
    name: 'CLD1015Panel',
    components: {
      ConnectionStatus,
      LaserStatus,
      TemperatureControl,
      CurrentControl
    },
    setup() {
      const isConnected = ref(false);
      const hasBeenConnected = ref(false);
      const tecEnabled = ref(false);
      
      const connectDevice = async () => {
        try {
          const response = await invoke('connect_cld1015');
          console.log('Connection response:', response);
          isConnected.value = true;
          hasBeenConnected.value = true;
          
          // Get initial TEC state
          const tecState = await invoke('get_cld1015_tec_state');
          tecEnabled.value = tecState as boolean;
        } catch (error) {
          console.error('Failed to connect to CLD1015:', error);
        }
      };
      
      const disconnectDevice = async () => {
        try {
          // In a real app, you'd have a disconnect command
          // await invoke('disconnect_cld1015');
          isConnected.value = false;
        } catch (error) {
          console.error('Failed to disconnect from CLD1015:', error);
        }
      };
      
      const onModeChanged = (mode: string) => {
        console.log('Operation mode changed to:', mode);
        // Additional logic if needed when mode changes
      };
      
      const onTecStateChanged = (enabled: boolean) => {
        tecEnabled.value = enabled;
      };
      
      onMounted(async () => {
        // Check if device is already connected
        try {
          const connected = await invoke('is_cld1015_connected');
          isConnected.value = connected as boolean;
          if (isConnected.value) {
            hasBeenConnected.value = true;
            
            // Get initial TEC state
            const tecState = await invoke('get_cld1015_tec_state');
            tecEnabled.value = tecState as boolean;
          }
        } catch (error) {
          console.error('Failed to check CLD1015 connection status:', error);
        }
      });
      
      return {
        isConnected,
        hasBeenConnected,
        tecEnabled,
        connectDevice,
        disconnectDevice,
        onModeChanged,
        onTecStateChanged
      };
    }
  });
  </script>
  
  <style scoped>
  .cld1015-panel {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
  }
  
  .controls-container {
    margin-top: 20px;
  }
  
  h2 {
    margin-bottom: 20px;
    color: #2c3e50;
  }
  </style>
  