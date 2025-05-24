import { ref } from 'vue';
import { invoke, Channel } from "@tauri-apps/api/core";
import type { ChatMessage, ChatEvent } from '../types/chat';

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
    let aiContent = "";
    try {
      const onEvent = new Channel<ChatEvent>();
      onEvent.onmessage = (message: any) => {
        if (message.event === "stopped") {
          const choices = (message.data && (message.data as any)["message"]["choices"]) || [];
          const content = choices[0]?.delta?.content || "";
          const finishReason = choices[0]?.finish_reason;
          console.log(content);
          if (content) {
            aiContent += content;
            const lastIndex = messages.value.length - 1;
            messages.value[lastIndex] = {
              ...messages.value[lastIndex],
              text: aiContent,
              isLoading: finishReason != "stop"
            };
            scrollToBottom();
          }
        }
      };     
      await invoke("make_chat", { question: text, onEvent });
      // 最终输出完成，确保 loading 状态被移除
      const lastIndex = messages.value.length - 1;
      messages.value[lastIndex] = { 
        ...messages.value[lastIndex],
        text: aiContent,
        isLoading: false 
      };
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