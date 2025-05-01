import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import type { ChatMessage } from '../types/chat';

export function useChat() {
  const messages = ref<ChatMessage[]>([
    { text: "你好！我是Marsho喵！", isUser: false }
  ]);
  const loading = ref(false);

  async function sendMessage(text: string) {
    if (loading.value) return;
    loading.value = true;
    messages.value.push({ text, isUser: true });
    messages.value.push({ text: "", isUser: false, isLoading: true });
    try {
      const result: string = await invoke("make_chat", { name: text });
      const lastIndex = messages.value.length - 1;
      messages.value[lastIndex] = { text: result, isUser: false };
    } catch (error: any) {
      console.error(error);
      const lastIndex = messages.value.length - 1;
      messages.value[lastIndex] = { text: "发生错误:" + error, isUser: false };
    } finally {
      loading.value = false;
    }
    
    scrollToBottom();
  }

  function scrollToBottom() {
    setTimeout(() => {
      const chatContainer = document.querySelector('.chat-messages');
      if (chatContainer) {
        chatContainer.scrollTop = chatContainer.scrollHeight;
      }
    }, 100);
  }

  return {
    messages,
    sendMessage,
    loading
  };
}