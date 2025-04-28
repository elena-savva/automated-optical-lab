<template>
    <div class="parameter-slider">
      <div class="parameter-header">
        <label>{{ label }}</label>
        <span>{{ value }} {{ unit }}</span>
      </div>
      <input 
        type="range" 
        :min="min" 
        :max="max" 
        :step="step" 
        v-model="localValue" 
        @change="onSliderChange"
      />
      <div class="range-labels">
        <span>{{ min }}</span>
        <span>{{ max }}</span>
      </div>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref, watch } from 'vue';
  
  export default defineComponent({
    name: 'ParameterSlider',
    props: {
      label: {
        type: String,
        required: true
      },
      value: {
        type: Number,
        required: true
      },
      min: {
        type: Number,
        required: true
      },
      max: {
        type: Number,
        required: true
      },
      step: {
        type: Number,
        default: 1
      },
      unit: {
        type: String,
        default: ''
      }
    },
    emits: ['update:value', 'change'],
    setup(props, { emit }) {
      const localValue = ref(props.value);
  
      watch(() => props.value, (newVal) => {
        localValue.value = newVal;
      });
  
      const onSliderChange = () => {
        emit('update:value', Number(localValue.value));
        emit('change', Number(localValue.value));
      };
  
      return {
        localValue,
        onSliderChange
      };
    }
  });
  </script>
  
  <style scoped>
  .parameter-slider {
    margin-bottom: 15px;
    width: 100%;
  }
  
  .parameter-header {
    display: flex;
    justify-content: space-between;
    margin-bottom: 5px;
  }
  
  input[type="range"] {
    width: 100%;
    margin: 0;
  }
  
  .range-labels {
    display: flex;
    justify-content: space-between;
    margin-top: 5px;
    font-size: 0.8em;
    color: #666;
  }
  </style>
  