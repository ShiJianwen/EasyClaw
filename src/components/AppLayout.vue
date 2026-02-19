<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRouter, useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import {
  MessageSquare,
  Home,
  Layers,
  Brain,
  Settings,
} from "lucide-vue-next";

const router = useRouter();
const route = useRoute();

const gatewayConnected = ref(false);

onMounted(async () => {
  const status = await invoke<boolean>("gateway_status").catch(() => false);
  gatewayConnected.value = status;
});

const menuItems = [
  { icon: Home, label: "首页", route: "/home" },
  { icon: MessageSquare, label: "对话", route: "/chat" },
  { icon: Layers, label: "技能", route: null },
  { icon: Brain, label: "记忆", route: null },
  { icon: Settings, label: "设置", route: null },
];

function isActive(itemRoute: string | null): boolean {
  if (!itemRoute) return false;
  return route.path === itemRoute;
}

function handleMenuClick(itemRoute: string | null) {
  if (itemRoute) {
    router.push(itemRoute);
  }
}
</script>

<template>
  <div
    data-testid="app-layout"
    class="w-screen h-screen bg-dark-900 flex flex-row overflow-hidden"
  >
    <!-- Left sidebar -->
    <aside
      data-testid="sidebar"
      class="w-44 h-full flex flex-col py-4 px-3 bg-dark-900/80 backdrop-blur-xl border-r border-white/5 flex-shrink-0"
    >
      <!-- Brand -->
      <div class="flex items-center gap-2.5 px-2 mb-5">
        <svg
          width="28"
          height="28"
          viewBox="0 0 64 64"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <defs>
            <linearGradient
              id="sidebarGrad"
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
          <circle cx="32" cy="32" r="22" fill="url(#sidebarGrad)" opacity="0.25" />
          <path
            d="M20 38 C20 28, 24 20, 28 16 C29 15, 30 16, 29 18 C28 22, 26 28, 26 34"
            stroke="url(#sidebarGrad)"
            stroke-width="3"
            stroke-linecap="round"
            fill="none"
          />
          <path
            d="M28 36 C28 26, 30 18, 32 14 C33 13, 34 14, 33 16 C32 20, 30 26, 30 32"
            stroke="url(#sidebarGrad)"
            stroke-width="3"
            stroke-linecap="round"
            fill="none"
          />
          <path
            d="M36 36 C36 26, 34 18, 32 14 C31 13, 30 14, 31 16 C32 20, 34 26, 34 32"
            stroke="url(#sidebarGrad)"
            stroke-width="3"
            stroke-linecap="round"
            fill="none"
          />
          <path
            d="M44 38 C44 28, 40 20, 36 16 C35 15, 34 16, 35 18 C36 22, 38 28, 38 34"
            stroke="url(#sidebarGrad)"
            stroke-width="3"
            stroke-linecap="round"
            fill="none"
          />
          <path
            d="M22 40 Q32 48 42 40"
            stroke="url(#sidebarGrad)"
            stroke-width="3"
            stroke-linecap="round"
            fill="none"
          />
        </svg>
        <span class="text-text-primary font-semibold text-sm">EasyClaw</span>
      </div>

      <!-- Menu items -->
      <nav class="flex-1 flex flex-col gap-1">
        <button
          v-for="(item, index) in menuItems"
          :key="index"
          data-testid="sidebar-menu-item"
          class="w-full h-9 rounded-lg flex items-center gap-3 px-3 transition-all duration-200 cursor-pointer text-left"
          :class="[
            isActive(item.route)
              ? 'bg-primary/20 text-primary-lighter'
              : 'text-text-muted hover:bg-white/5 hover:text-text-secondary',
            !item.route ? 'opacity-40 cursor-not-allowed' : '',
          ]"
          :disabled="!item.route"
          @click="handleMenuClick(item.route)"
        >
          <component :is="item.icon" class="w-4 h-4 flex-shrink-0" />
          <span class="text-sm truncate">{{ item.label }}</span>
        </button>
      </nav>

      <!-- Gateway status indicator -->
      <div class="mt-auto flex items-center gap-2 px-3 py-2">
        <span
          class="w-2 h-2 rounded-full flex-shrink-0"
          :class="gatewayConnected ? 'bg-success' : 'bg-danger'"
        />
        <span class="text-text-muted text-[11px] truncate">
          {{ gatewayConnected ? '已连接' : '未连接' }}
        </span>
      </div>
    </aside>

    <!-- Right: main content + status bar -->
    <div class="flex-1 flex flex-col overflow-hidden">
      <!-- Main content area -->
      <main data-testid="main-content" class="flex-1 overflow-hidden flex flex-col">
        <router-view />
      </main>

      <!-- Bottom status bar -->
      <div
        data-testid="status-bar"
        class="h-7 flex items-center justify-between px-4 text-[11px] bg-dark-900/60 border-t border-white/5"
      >
        <div class="flex items-center gap-2">
          <span
            class="w-1.5 h-1.5 rounded-full"
            :class="gatewayConnected ? 'bg-success' : 'bg-danger'"
          />
          <span class="text-text-muted">
            ZeroClaw {{ gatewayConnected ? '已连接' : '未连接' }}
          </span>
        </div>
        <span class="text-text-muted">v0.1.0</span>
      </div>
    </div>
  </div>
</template>
