<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { AppConfig } from '../models/config';
import { ref } from 'vue';

defineProps<{
  modelValue: boolean
}>();


async function readConfig() {
  return await invoke('get_configs') as AppConfig;
}

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  (e: 'save', settings: any): void
}>();

const config = await readConfig();

const baseUrl = ref(config.marsho.base_url || '');
const apiKey = ref(config.marsho.api_key || '');
const stream = ref(config.marsho.stream || false);
const systemPrompt = ref(config.marsho.system_prompt || '');

const save = async () => {
  config.marsho.base_url = baseUrl.value;
  config.marsho.api_key = apiKey.value;
  config.marsho.stream = stream.value;
  config.marsho.system_prompt = systemPrompt.value;
  emit('save', await invoke('save_configs', { marshoConfig: config.marsho }));
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
				<div class="settings-item-input">
					<label>Base URL:</label>
					<input v-model="baseUrl" placeholder="请输入Base URL" />
				</div>
				<div class="settings-item-input">
					<label>API Key:</label>
					<input v-model="apiKey" placeholder="请输入API Key" />
				</div>
				<div class="settings-item-checkbox">
					<label>流式调用：</label>
					<input type="checkbox" v-model="stream" />
				</div>
				<div class="settings-item-input">
					<label>系统提示：</label>
					<input v-model="systemPrompt" placeholder="请输入系统提示" />
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

/*
.settings-item label {
  display: block;
  margin-bottom: 5px;
}

.settings-item input {
  width: calc(100% - 16px - 2px);
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
}
*/

.settings-item-input, .settings-item-checkbox {
	margin-bottom: 1em;
}

.settings-item-input label {
	display: block;
	margin-bottom: 5px;
}

.settings-item-input input {
	width: calc(100% - 16px - 2px);
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.settings-item-checkbox input {
	vertical-align: -2px;
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