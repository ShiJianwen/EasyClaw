<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import {
  MessageSquare,
  Layers,
  Brain,
  Settings,
} from "lucide-vue-next";

const gatewayConnected = ref(false);

const features = [
  {
    icon: MessageSquare,
    title: "对话助手",
    description: "与 AI 助手自然对话，获取智能回答和建议",
  },
  {
    icon: Layers,
    title: "技能管理",
    description: "管理和配置已安装的技能包，扩展助手能力",
  },
  {
    icon: Brain,
    title: "记忆管理",
    description: "查看和管理 AI 助手的长期记忆与用户画像",
  },
  {
    icon: Settings,
    title: "系统设置",
    description: "自定义模型、网关端口等核心配置参数",
  },
];

onMounted(async () => {
  const status = await invoke<boolean>("gateway_status").catch(() => false);
  gatewayConnected.value = status;
});
</script>

<template>
  <div class="w-screen h-screen bg-dark-900 flex flex-col overflow-hidden">
    <!-- Top navigation bar -->
    <nav
      data-testid="navbar"
      class="fixed top-0 left-0 right-0 h-14 glass-nav flex items-center justify-between px-5 z-50"
    >
      <div class="flex items-center gap-3">
        <svg
          width="28"
          height="28"
          viewBox="0 0 64 64"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <defs>
            <linearGradient
              id="navGrad"
              x1="0"
              y1="0"
              x2="64"
              y2="64"
              gradientUnits="userSpaceOnUse"
            >
              <stop offset="0%" stop-color="#6366F1" />
              <stop offset="100%" stop-color="#A78BFA" />
            </linearGradient>
          </defs>
          <circle cx="32" cy="32" r="22" fill="url(#navGrad)" opacity="0.25" />
          <path
            d="M20 38 C20 28, 24 20, 28 16 C29 15, 30 16, 29 18 C28 22, 26 28, 26 34"
            stroke="url(#navGrad)"
            stroke-width="3"
            stroke-linecap="round"
            fill="none"
          />
          <path
            d="M28 36 C28 26, 30 18, 32 14 C33 13, 34 14, 33 16 C32 20, 30 26, 30 32"
            stroke="url(#navGrad)"
            stroke-width="3"
            stroke-linecap="round"
            fill="none"
          />
          <path
            d="M36 36 C36 26, 34 18, 32 14 C31 13, 30 14, 31 16 C32 20, 34 26, 34 32"
            stroke="url(#navGrad)"
            stroke-width="3"
            stroke-linecap="round"
            fill="none"
          />
          <path
            d="M44 38 C44 28, 40 20, 36 16 C35 15, 34 16, 35 18 C36 22, 38 28, 38 34"
            stroke="url(#navGrad)"
            stroke-width="3"
            stroke-linecap="round"
            fill="none"
          />
          <path
            d="M22 40 Q32 48 42 40"
            stroke="url(#navGrad)"
            stroke-width="3"
            stroke-linecap="round"
            fill="none"
          />
        </svg>
        <span class="text-text-primary font-semibold text-base">EasyClaw</span>
      </div>
      <button
        class="w-8 h-8 rounded-lg bg-white/5 flex items-center justify-center hover:bg-white/10 transition-base cursor-pointer"
      >
        <Settings class="w-4 h-4 text-text-secondary" />
      </button>
    </nav>

    <!-- Main content area -->
    <main class="flex-1 pt-14 pb-8 px-6 overflow-y-auto">
      <!-- Welcome banner -->
      <div
        data-testid="welcome-banner"
        class="mt-6 rounded-2xl p-8 relative overflow-hidden bg-gradient-to-br from-dark-700 via-dark-800 to-dark-900 border border-white/5"
      >
        <div
          class="absolute top-0 right-0 w-64 h-64 bg-primary/10 rounded-full blur-3xl -translate-y-1/2 translate-x-1/3 pointer-events-none"
        />
        <div
          class="absolute bottom-0 left-1/3 w-40 h-40 bg-primary-light/5 rounded-full blur-2xl translate-y-1/2 pointer-events-none"
        />
        <div class="relative z-10">
          <h2 class="text-2xl font-bold text-text-primary mb-2">
            欢迎使用 EasyClaw
          </h2>
          <p class="text-text-secondary text-sm max-w-md">
            您的 AI 智能助手已就绪。选择下方功能开始体验，或直接与助手对话。
          </p>
        </div>
      </div>

      <!-- Feature cards grid -->
      <div class="grid grid-cols-2 gap-4 mt-6">
        <div
          v-for="(feature, index) in features"
          :key="index"
          data-testid="feature-card"
          class="glass-card p-5 group cursor-pointer hover:-translate-y-1 hover:shadow-[0_8px_30px_rgba(99,102,241,0.12)] hover:border-primary/20 transition-all duration-300"
        >
          <div
            class="w-10 h-10 rounded-xl bg-gradient-to-br from-primary/20 to-primary-light/10 flex items-center justify-center mb-4 group-hover:from-primary/30 group-hover:to-primary-light/20 transition-all duration-300"
          >
            <component
              :is="feature.icon"
              class="w-5 h-5 text-primary-lighter"
            />
          </div>
          <h3
            class="text-text-primary font-semibold text-sm mb-1.5"
          >
            {{ feature.title }}
          </h3>
          <p class="text-text-muted text-xs leading-relaxed">
            {{ feature.description }}
          </p>
        </div>
      </div>
    </main>

    <!-- Bottom status bar -->
    <div
      data-testid="status-bar"
      class="fixed bottom-0 left-0 right-0 h-8 glass-nav flex items-center justify-between px-4 text-[11px] z-50"
    >
      <div class="flex items-center gap-2">
        <span
          class="w-2 h-2 rounded-full"
          :class="gatewayConnected ? 'bg-success' : 'bg-danger'"
        />
        <span class="text-text-muted">
          Gateway {{ gatewayConnected ? '已连接' : '未连接' }}
        </span>
      </div>
      <span class="text-text-muted">v0.1.0</span>
    </div>
  </div>
</template>
