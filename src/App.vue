<template>
  <div class="app-container">
    <header>
      <h1>Optical Lab Control Suite</h1>
    </header>
    
    <main>
      <div class="tabs">
        <button 
          v-for="tab in tabs" 
          :key="tab.id"
          @click="currentTab = tab.id"
          :class="{ active: currentTab === tab.id }"
          class="tab-button"
        >
          {{ tab.name }}
        </button>
      </div>
      
      <div class="tab-content">
        <keep-alive>
          <component :is="currentTabComponent"></component>
        </keep-alive>
      </div>
    </main>
    
    <footer>
      <p>© 2025 Optical Lab Suite</p>
    </footer>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed } from 'vue';
import CLD1015Panel from './components/cld1015/CLD1015Panel.vue';
import MPM210HPanel from './components/mpm210h/MPM210HPanel.vue';
import AutomationSetup from './components/experiments/AutomationSetup.vue';

export default defineComponent({
  name: 'App',
  components: {
    CLD1015Panel,
    MPM210HPanel,
    AutomationSetup
  },
  setup() {
    const tabs = [
      { id: 'cld1015', name: 'CLD1015 Laser Controller' },
      { id: 'mpm210h', name: 'MPM-210H Power Meter' },
      { id: 'experiment', name: 'Experiment Automation' }
    ];
    
    const currentTab = ref('cld1015');
    
    const currentTabComponent = computed(() => {
      switch (currentTab.value) {
        case 'cld1015':
          return CLD1015Panel;
        case 'mpm210h':
          return MPM210HPanel;
        case 'experiment':
          return AutomationSetup;
        default:
          return CLD1015Panel;
      }
    });
    
    return {
      tabs,
      currentTab,
      currentTabComponent
    };
  }
});
</script>

<style>
/* Global styles */
body {
  font-family: Arial, sans-serif;
  margin: 0;
  padding: 0;
  color: #333;
  background-color: #f0f2f5;
}

* {
  box-sizing: border-box;
}

.app-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

header {
  padding: 20px 0;
  border-bottom: 1px solid #e0e0e0;
}

h1 {
  margin: 0;
  color: #2c3e50;
}

main {
  flex: 1;
  padding: 20px 0;
}

.tabs {
  display: flex;
  border-bottom: 1px solid #e0e0e0;
  margin-bottom: 20px;
}

.tab-button {
  padding: 10px 20px;
  background: none;
  border: none;
  cursor: pointer;
  font-size: 16px;
  color: #666;
  border-bottom: 3px solid transparent;
}

.tab-button.active {
  color: #2196F3;
  border-bottom-color: #2196F3;
  font-weight: bold;
}

.tab-content {
  padding: 10px;
}

footer {
  padding: 20px 0;
  text-align: center;
  color: #666;
  font-size: 14px;
  border-top: 1px solid #e0e0e0;
}
</style>
