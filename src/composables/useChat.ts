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
        if (message.event === "outputing") {
          const choices = (message.data && (message.data as any)["message"]["choices"]) || [];
          const content = choices[0]?.delta?.content || "";
          if (content) {
            aiContent += content;
            const lastIndex = messages.value.length - 1;
            messages.value[lastIndex] = {
              ...messages.value[lastIndex],
              text: aiContent,
              isLoading: true
            };
            scrollToBottom();
          }
        } else if (message.event === "finished") {
          console.log("输出完成：", message.data);
          // 当收到 finished 事件时，移除加载状态
          const lastIndex = messages.value.length - 1;
          messages.value[lastIndex] = {
            ...messages.value[lastIndex],
            text: aiContent,
            isLoading: false
          };
          scrollToBottom();
        }
      };     
      await invoke("make_chat", { question: text, onEvent });
      // 最终结果已经在 finished 事件中处理，这里不需要重复设置
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