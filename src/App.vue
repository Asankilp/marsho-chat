<script setup lang="ts">
import { useChat } from './composables/useChat';
import ChatMessage from './components/ChatMessage.vue';
import ChatInput from './components/ChatInput.vue';
import Settings from './components/Settings.vue';
import { ref } from 'vue';

const { messages, sendMessage, loading } = useChat();
const showSettings = ref(false);

const handleSettingsSave = (settings: any) => {
  // 这里可以处理设置保存后的逻辑
  console.log('Settings saved:', settings);
};
</script>

<template>
  <div class="chat-container">
    <header class="chat-header">
      <img src="./assets/marsho-new.svg" height="48px" width="90px"/>
      <h1>Marsho Chat</h1>
      <button class="settings-button" @click="showSettings = true">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="3"></circle>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
        </svg>
      </button>
    </header>
    
    <main class="chat-messages">
      <ChatMessage 
        v-for="(message, index) in messages" 
        :key="index" 
        :message="message" 
      />
    </main>
    
    <footer class="chat-footer">
      <ChatInput @send="sendMessage" :loading="loading" />
    </footer>

    <Suspense>
      <Settings v-model="showSettings" @save="handleSettingsSave" />
      <template #fallback>
        <div class="loading-settings">加载设置中...</div>
      </template>
    </Suspense>
  </div>
</template>

<style>
:root {
  /* --max-chat-width: 1200px; */
  --header-height: 60px;
  --footer-height: 80px;
}

body {
  margin: 0;
  padding: 0;
  /* font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
    Ubuntu, Cantarell, 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif; */
}

.chat-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  max-width: var(--max-chat-width);
  margin: 0 auto;
  background: #ffffff;
}

.chat-header {
  height: var(--header-height);
  padding: 0 1rem;
  display: flex;
  align-items: center;
  background: #f8f9fa;
  border-bottom: 1px solid #e9ecef;
  position: fixed;
  top: 0;
  width: 100%;
  max-width: var(--max-chat-width);
  z-index: 100;
  justify-content: space-between;
}

.chat-header h1 {
  font-size: 1.5rem;
  margin: 0;
  color: #333;
}

.chat-messages {
  flex: 1;
  overflow-y: auto;
  padding: 1rem;
  margin: var(--header-height) 0 var(--footer-height) 0;
  scroll-behavior: smooth;
}

.chat-footer {
  position: fixed;
  bottom: 0;
  width: 100%;
  max-width: var(--max-chat-width);
  height: var(--footer-height);
  background: #ffffff;
  border-top: 1px solid #e9ecef;
  z-index: 100;
}

.settings-button {
  background: none;
  border: none;
  padding: 8px;
  cursor: pointer;
  color: inherit;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: background-color 0.2s;
}

.settings-button:hover {
  background: rgba(0, 0, 0, 0.1);
}

.loading-settings {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  padding: 1rem;
  background: rgba(0, 0, 0, 0.7);
  color: white;
  border-radius: 8px;
}

@media (max-width: 768px) {
  :root {
    --header-height: 50px;
    --footer-height: 70px;
  }

  .chat-header h1 {
    font-size: 1.2rem;
  }

  .chat-messages {
    padding: 0.5rem;
  }
}

@media (prefers-color-scheme: dark) {
  body {
    background: #1a1a1a;
    color: #ffffff;
  }

  .chat-container {
    background: #1a1a1a;
  }

  .chat-header {
    background: #2f2f2f;
    border-bottom-color: #404040;
  }

  .chat-header h1 {
    color: #ffffff;
  }

  .chat-footer {
    background: #1a1a1a;
    border-top-color: #404040;
  }

  .settings-button:hover {
    background: rgba(255, 255, 255, 0.1);
  }
}

/* 适配较小屏幕的安全区域 */
@supports (padding: max(0px)) {
  .chat-header {
    padding-left: max(1rem, env(safe-area-inset-left));
    padding-right: max(1rem, env(safe-area-inset-right));
  }

  .chat-footer {
    padding-bottom: env(safe-area-inset-bottom);
  }
}
</style>