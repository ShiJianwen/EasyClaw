<script setup lang="ts">
import { ref, nextTick, watch, onUnmounted } from "vue";
import {
  Trash2,
  Send,
  Square,
  MessageSquare,
  AlertCircle,
} from "lucide-vue-next";
import { useChat } from "../composables/useChat";

const { messages, isLoading, error, sendMessage, abortResponse, clearMessages } = useChat();

const inputText = ref("");
const messagesEndRef = ref<HTMLElement | null>(null);

async function handleSend() {
  const text = inputText.value.trim();
  if (!text || isLoading.value) return;
  inputText.value = "";
  await sendMessage(text);
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === "Enter" && !event.shiftKey) {
    event.preventDefault();
    handleSend();
  }
}

function formatTime(timestamp: number): string {
  const date = new Date(timestamp);
  return date.toLocaleTimeString("zh-CN", {
    hour: "2-digit",
    minute: "2-digit",
  });
}

function scrollToBottom() {
  messagesEndRef.value?.scrollIntoView({ behavior: "smooth" });
}

watch(
  () => messages.value.length,
  () => {
    nextTick(scrollToBottom);
  }
);

watch(
  () => messages.value.at(-1)?.content,
  () => {
    nextTick(scrollToBottom);
  }
);

onUnmounted(() => {
  abortResponse();
});
</script>

<template>
  <div class="flex flex-col h-full">
    <!-- Chat header -->
    <div class="flex items-center justify-between px-4 h-12 border-b border-white/5">
      <span class="text-text-primary font-semibold text-sm">对话助手</span>
      <button
        data-testid="clear-button"
        class="w-8 h-8 rounded-lg bg-white/5 flex items-center justify-center hover:bg-white/10 transition-base cursor-pointer"
        @click="clearMessages"
      >
        <Trash2 class="w-4 h-4 text-text-secondary" />
      </button>
    </div>

    <!-- Messages area -->
    <div class="flex-1 overflow-y-auto px-4 py-4 space-y-4">
      <!-- Empty state -->
      <div
        v-if="messages.length === 0"
        data-testid="empty-state"
        class="flex flex-col items-center justify-center h-full gap-4"
      >
        <div class="w-16 h-16 rounded-2xl bg-white/5 flex items-center justify-center">
          <MessageSquare class="w-8 h-8 text-text-muted opacity-50" />
        </div>
        <div class="text-center">
          <p class="text-text-secondary text-sm font-medium mb-1">有什么可以帮您？</p>
          <p class="text-text-muted text-xs">输入消息开始与 AI 助手对话</p>
        </div>
      </div>

      <!-- Message bubbles -->
      <template v-for="msg in messages" :key="msg.id">
        <!-- User message -->
        <div
          v-if="msg.role === 'user'"
          data-testid="message-user"
          class="flex justify-end"
        >
          <div class="max-w-[80%]">
            <div class="chat-bubble-user px-4 py-2.5 text-sm text-white leading-relaxed whitespace-pre-wrap">
              {{ msg.content }}
            </div>
            <p class="text-text-muted text-[11px] mt-1 text-right">{{ formatTime(msg.timestamp) }}</p>
          </div>
        </div>

        <!-- Assistant message -->
        <div
          v-else
          data-testid="message-assistant"
          class="flex justify-start"
        >
          <div class="max-w-[80%]">
            <div
              v-if="msg.status === 'error'"
              data-testid="message-error"
              class="chat-bubble-ai border border-danger/30 px-4 py-2.5 text-sm leading-relaxed whitespace-pre-wrap"
            >
              <div class="flex items-start gap-2">
                <AlertCircle class="w-4 h-4 text-danger flex-shrink-0 mt-0.5" />
                <span class="text-danger/90">{{ msg.content }}</span>
              </div>
            </div>
            <div
              v-else
              class="chat-bubble-ai px-4 py-2.5 text-sm text-text-primary leading-relaxed whitespace-pre-wrap"
            >
              {{ msg.content }}<span
                v-if="msg.status === 'streaming'"
                class="inline-block w-0.5 h-4 bg-primary-lighter ml-0.5 animate-pulse align-middle"
              />
            </div>
            <p class="text-text-muted text-[11px] mt-1">{{ formatTime(msg.timestamp) }}</p>
          </div>
        </div>
      </template>

      <!-- Error bar -->
      <div
        v-if="error"
        data-testid="error-bar"
        class="glass-card border-danger/20 p-3 flex items-center gap-3"
      >
        <AlertCircle class="w-4 h-4 text-danger flex-shrink-0" />
        <span class="text-text-secondary text-xs flex-1">{{ error }}</span>
        <button
          class="text-xs text-primary bg-primary/10 px-3 py-1 rounded-full hover:bg-primary/20 transition-base cursor-pointer"
          @click="() => { const last = messages.filter(m => m.role === 'user').at(-1); if (last) sendMessage(last.content); }"
        >
          重试
        </button>
      </div>

      <div ref="messagesEndRef" />
    </div>

    <!-- Input area -->
    <div class="px-4 py-3 glass-nav border-t border-white/5">
      <div class="flex items-end gap-3">
        <textarea
          data-testid="chat-input"
          v-model="inputText"
          :disabled="isLoading"
          placeholder="输入消息..."
          rows="1"
          class="flex-1 bg-white/5 border border-white/10 rounded-xl px-4 py-2.5 text-sm text-text-primary placeholder-text-muted resize-none outline-none focus:border-primary/30 focus:ring-1 focus:ring-primary/20 transition-base max-h-28 overflow-y-auto"
          @keydown="handleKeydown"
        />

        <!-- Send / Stop button -->
        <button
          v-if="!isLoading"
          data-testid="send-button"
          :disabled="!inputText.trim()"
          class="w-10 h-10 rounded-xl flex items-center justify-center flex-shrink-0 transition-all duration-300 cursor-pointer"
          :class="
            inputText.trim()
              ? 'bg-gradient-to-r from-primary to-primary-light hover:shadow-[0_0_20px_rgba(99,102,241,0.3)]'
              : 'bg-white/5 opacity-40 cursor-not-allowed'
          "
          @click="handleSend"
        >
          <Send class="w-4 h-4 text-white" />
        </button>
        <button
          v-else
          data-testid="stop-button"
          class="w-10 h-10 rounded-xl bg-danger/20 flex items-center justify-center flex-shrink-0 hover:bg-danger/30 transition-base cursor-pointer"
          @click="abortResponse"
        >
          <Square class="w-4 h-4 text-danger" />
        </button>
      </div>
    </div>
  </div>
</template>
