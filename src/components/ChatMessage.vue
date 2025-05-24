<script setup lang="ts">
import type { ChatMessage } from '../types/chat';

defineProps<{
  message: ChatMessage;
}>();
</script>

<template>
  <div :class="['message', message.isUser ? 'user-message' : 'bot-message']">
    <div class="message-content" :class="{ 'with-loading': message.isLoading }">
      {{ message.text }}
      <div class="loading-dots" v-if="message.isLoading">
        <span></span>
        <span></span>
        <span></span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.message {
  display: flex;
  margin-bottom: 0.75rem;
  padding: 0 0.5rem;
}

.message-content {
  max-width: min(70%, 600px);
  padding: 0.75rem 1rem;
  border-radius: 1rem;
  word-break: break-word;
  white-space: pre-wrap;
  font-size: 1rem;
  line-height: 1.5;
  display: flex;
  align-items: center;
  gap: 8px;
}

.message-content.with-loading {
  min-width: 60px;
}

.user-message {
  justify-content: flex-end;
}

.user-message .message-content {
  background: #007AFF;
  color: white;
  border-radius: 1rem 1rem 0 1rem;
}

.bot-message .message-content {
  background: #E9E9EB;
  color: black;
  border-radius: 1rem 1rem 1rem 0;
}

.loading-message {
  min-width: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.loading-dots {
  display: flex;
  gap: 4px;
}

.loading-dots span {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background-color: #666;
  animation: bounce 1.4s infinite ease-in-out both;
}

.loading-dots span:nth-child(1) {
  animation-delay: -0.32s;
}

.loading-dots span:nth-child(2) {
  animation-delay: -0.16s;
}

@keyframes bounce {
  0%, 80%, 100% {
    transform: scale(0);
  }
  40% {
    transform: scale(1);
  }
}

@media (prefers-color-scheme: dark) {
  .bot-message .message-content {
    background: #3a3a3a;
    color: white;
  }
  
  .loading-dots span {
    background-color: #999;
  }
}

@media (max-width: 768px) {
  .message {
    margin-bottom: 0.5rem;
    padding: 0 0.25rem;
  }

  .message-content {
    max-width: 85%;
    padding: 0.5rem 0.75rem;
    font-size: 0.95rem;
  }
}

@media (max-width: 480px) {
  .message-content {
    max-width: 90%;
    font-size: 0.9rem;
  }

	.user-message .message-content {
		border-radius: 0.75rem 0.75rem 0 0.75rem;
	}

	.bot-message .message-content {
		border-radius: 0.75rem 0.75rem 0.75rem 0;
	}
}
</style>