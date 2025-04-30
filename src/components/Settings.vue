<script setup lang="ts">
import { ref } from 'vue';

defineProps<{
  modelValue: boolean
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  (e: 'save', settings: any): void
}>();

const apiKey = ref(localStorage.getItem('apiKey') || '');

const save = () => {
  localStorage.setItem('apiKey', apiKey.value);
  emit('save', { apiKey: apiKey.value });
  emit('update:modelValue', false);
};

const close = () => {
  emit('update:modelValue', false);
};
</script>

<template>
  <div v-if="modelValue" class="settings-overlay">
    <div class="settings-modal">
      <h2>设置</h2>
      <div class="settings-content">
        <div class="settings-item">
          <label>API Key:</label>
          <input type="password" v-model="apiKey" placeholder="请输入API Key" />
        </div>
      </div>
      <div class="settings-actions">
        <button @click="save">保存</button>
        <button @click="close">取消</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.settings-modal {
  background: #fff;
  padding: 20px;
  border-radius: 8px;
  min-width: 300px;
}

.settings-content {
  margin: 20px 0;
}

.settings-item {
  margin-bottom: 15px;
}

.settings-item label {
  display: block;
  margin-bottom: 5px;
}

.settings-item input {
  width: 100%;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.settings-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.settings-actions button {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.settings-actions button:first-child {
  background: #007AFF;
  color: white;
}

.settings-actions button:last-child {
  background: #f5f5f5;
}

@media (prefers-color-scheme: dark) {
  .settings-modal {
    background: #2f2f2f;
    color: #fff;
  }

  .settings-item input {
    background: #1a1a1a;
    border-color: #404040;
    color: #fff;
  }

  .settings-actions button:last-child {
    background: #404040;
    color: #fff;
  }
}
</style>