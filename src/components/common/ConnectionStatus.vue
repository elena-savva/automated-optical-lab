<template>
    <div class="connection-status">
      <div class="status-indicator" :class="{ connected: isConnected, disconnected: !isConnected }"></div>
      <span>{{ deviceName }}: {{ isConnected ? 'Connected' : 'Disconnected' }}</span>
      <button 
        class="connect-button" 
        @click="toggleConnection" 
        :disabled="isConnecting"
      >
        {{ isConnected ? 'Disconnect' : 'Connect' }}
      </button>
      <div v-if="isConnecting" class="connecting-spinner"></div>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref } from 'vue';
  
  export default defineComponent({
    name: 'ConnectionStatus',
    props: {
      deviceName: {
        type: String,
        required: true
      },
      isConnected: {
        type: Boolean,
        required: true
      }
    },
    emits: ['connect', 'disconnect'],
    setup(props, { emit }) {
      const isConnecting = ref(false);
  
      const toggleConnection = async () => {
        isConnecting.value = true;
        
        try {
          if (props.isConnected) {
            emit('disconnect');
          } else {
            emit('connect');
          }
        } finally {
          // In a real app, this would be handled by watching the prop change
          setTimeout(() => {
            isConnecting.value = false;
          }, 500);
        }
      };
  
      return {
        isConnecting,
        toggleConnection
      };
    }
  });
  </script>
  
  <style scoped>
  .connection-status {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px;
    border-radius: 4px;
    background-color: #f5f5f5;
  }
  
  .status-indicator {
    width: 12px;
    height: 12px;
    border-radius: 50%;
  }
  
  .connected {
    background-color: #4CAF50;
  }
  
  .disconnected {
    background-color: #F44336;
  }
  
  .connect-button {
    padding: 5px 10px;
    border-radius: 4px;
    border: none;
    background-color: #2196F3;
    color: white;
    cursor: pointer;
  }
  
  .connect-button:disabled {
    background-color: #cccccc;
    cursor: not-allowed;
  }
  
  .connecting-spinner {
    width: 20px;
    height: 20px;
    border: 2px solid #f3f3f3;
    border-top: 2px solid #3498db;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
  
  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
  </style>
  