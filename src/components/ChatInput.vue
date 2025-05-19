<script setup lang="ts">
import { ref } from 'vue';

const props = defineProps<{
  loading?: boolean
}>();

const emit = defineEmits<{
  'send': [message: string]
}>();

const inputMessage = ref('');

function sendMessage() {
  if (!inputMessage.value.trim() || props.loading) return;
  emit('send', inputMessage.value);
  inputMessage.value = '';
}
</script>

<template>
  <form class="chat-input" @submit.prevent="sendMessage">
    <input 
      v-model="inputMessage" 
      type="text"
      placeholder="输入消息..." 
      @keyup.enter="sendMessage"
      class="input-field"
      :disabled="loading"
    />
    <button type="submit" class="send-button" :disabled="loading">发送</button>
  </form>
</template>

<style scoped>
.chat-input {
  display: flex;
  padding: 1rem;
  gap: 0.75rem;
  background: #f5f5f5;
  height: 100%;
  box-sizing: border-box;
}

.input-field {
  flex: 1;
  padding: 0.8rem 1.2rem;
  border: 1px solid #ddd;
  border-radius: 1.5rem;
  outline: none;
  font-size: 1rem;
  background: #ffffff;
  transition: border-color 0.2s, box-shadow 0.2s;
}

.input-field:focus {
  border-color: #007AFF;
  box-shadow: 0 0 0 2px rgba(0, 122, 255, 0.1);
}

.send-button {
  padding: 0.8rem 1.5rem;
  background: #007AFF;
  color: white;
  border: none;
  border-radius: 1.5rem;
  cursor: pointer;
  font-size: 1rem;
  font-weight: 500;
  transition: background-color 0.2s, transform 0.1s;
  white-space: nowrap;
  opacity: 1;
  &:disabled {
    background: #007AFF;
    opacity: 0.5;
    cursor: not-allowed;
  }
}

.send-button:hover {
  background: #0056b3;
}

.send-button:active {
  transform: scale(0.98);
}

@media (max-width: 768px) {
  .chat-input {
    padding: 0.75rem;
    gap: 0.5rem;
  }

  .input-field {
    padding: 0.6rem 1rem;
    font-size: 0.95rem;
		border-radius: 1rem;
  }

  .send-button {
    padding: 0.6rem 1.2rem;
    font-size: 0.95rem;
		border-radius: 1rem;
  }
}

@media (max-width: 480px) {
  .chat-input {
    padding: 0.5rem;
  }

  .input-field {
    padding: 0.5rem 0.8rem;
    font-size: 0.9rem;
		border-radius: 0.5rem;
  }

  .send-button {
    padding: 0.5rem 1rem;
    font-size: 0.9rem;
		border-radius: 0.5rem;
  }
}

@media (prefers-color-scheme: dark) {
  .chat-input {
    background: #2f2f2f;
  }
  
  .input-field {
    background: #3a3a3a;
    color: white;
    border-color: #555;
  }
  
  .input-field:focus {
    border-color: #007AFF;
    box-shadow: 0 0 0 2px rgba(0, 122, 255, 0.2);
  }
}
</style>