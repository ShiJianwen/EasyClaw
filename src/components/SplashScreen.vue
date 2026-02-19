<script setup lang="ts">
import type { InitStatus } from "../composables/useInitialization";

interface Props {
  status: InitStatus;
  progress: number;
  message: string;
  error: string | null;
}

defineProps<Props>();
defineEmits<{ retry: [] }>();
</script>

<template>
  <div
    class="w-screen h-screen bg-dark-900 flex flex-col items-center justify-center relative overflow-hidden"
  >
    <!-- Background glow effects -->
    <div
      class="absolute top-1/3 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[500px] h-[500px] rounded-full bg-primary/10 animate-glow-pulse pointer-events-none"
    />
    <div
      class="absolute bottom-1/4 right-1/4 w-[300px] h-[300px] rounded-full bg-primary-light/5 animate-glow-pulse pointer-events-none"
      style="animation-delay: 1s"
    />

    <!-- Logo and brand -->
    <div class="relative z-10 flex flex-col items-center">
      <!-- Logo SVG -->
      <div class="mb-6 relative">
        <div
          class="absolute inset-0 w-20 h-20 bg-primary/20 rounded-full blur-xl animate-glow-pulse"
        />
        <svg
          width="80"
          height="80"
          viewBox="0 0 64 64"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          class="relative z-10"
        >
          <defs>
            <linearGradient
              id="splashGrad"
              x1="0"
              y1="0"
              x2="64"
              y2="64"
              gradientUnits="userSpaceOnUse"
            >
              <stop offset="0%" stop-color="#6366F1" />
              <stop offset="50%" stop-color="#8B5CF6" />
              <stop offset="100%" stop-color="#A78BFA" />
            </linearGradient>
          </defs>
          <circle cx="32" cy="32" r="30" fill="url(#splashGrad)" opacity="0.15" />
          <circle cx="32" cy="32" r="22" fill="url(#splashGrad)" opacity="0.25" />
          <path
            d="M20 38 C20 28, 24 20, 28 16 C29 15, 30 16, 29 18 C28 22, 26 28, 26 34"
            stroke="url(#splashGrad)"
            stroke-width="3"
            stroke-linecap="round"
            fill="none"
          />
          <path
            d="M28 36 C28 26, 30 18, 32 14 C33 13, 34 14, 33 16 C32 20, 30 26, 30 32"
            stroke="url(#splashGrad)"
            stroke-width="3"
            stroke-linecap="round"
            fill="none"
          />
          <path
            d="M36 36 C36 26, 34 18, 32 14 C31 13, 30 14, 31 16 C32 20, 34 26, 34 32"
            stroke="url(#splashGrad)"
            stroke-width="3"
            stroke-linecap="round"
            fill="none"
          />
          <path
            d="M44 38 C44 28, 40 20, 36 16 C35 15, 34 16, 35 18 C36 22, 38 28, 38 34"
            stroke="url(#splashGrad)"
            stroke-width="3"
            stroke-linecap="round"
            fill="none"
          />
          <path
            d="M22 40 Q32 48 42 40"
            stroke="url(#splashGrad)"
            stroke-width="3"
            stroke-linecap="round"
            fill="none"
          />
        </svg>
      </div>

      <!-- Brand name -->
      <h1 class="text-3xl font-bold gradient-text mb-2">EasyClaw</h1>
      <p class="text-text-muted text-sm mb-10">OpenClaw 智能助手客户端</p>

      <!-- Progress section (shown when not error) -->
      <div v-if="status !== 'error'" class="w-80 flex flex-col items-center">
        <!-- Progress bar -->
        <div
          data-testid="progress-bar"
          class="w-full h-1.5 bg-white/5 rounded-full overflow-hidden mb-4"
        >
          <div
            class="h-full bg-gradient-to-r from-primary to-primary-lighter rounded-full relative transition-all duration-500 ease-out"
            :style="{ width: `${progress}%` }"
          >
            <div
              class="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent animate-shimmer"
            />
          </div>
        </div>

        <!-- Status message -->
        <p class="text-text-secondary text-xs">{{ message }}</p>
      </div>

      <!-- Error section -->
      <div v-else class="w-80 flex flex-col items-center">
        <div
          class="glass-card px-6 py-5 w-full flex flex-col items-center border-danger/20 mb-6"
        >
          <div class="w-10 h-10 rounded-full bg-danger/10 flex items-center justify-center mb-3">
            <svg
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="#EF4444"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <circle cx="12" cy="12" r="10" />
              <line x1="15" y1="9" x2="9" y2="15" />
              <line x1="9" y1="9" x2="15" y2="15" />
            </svg>
          </div>
          <p class="text-text-primary text-sm font-medium mb-1">初始化失败</p>
          <p class="text-text-muted text-xs text-center">{{ error }}</p>
        </div>

        <button
          data-testid="retry-button"
          class="px-8 py-2.5 bg-gradient-to-r from-primary to-primary-light text-white text-sm font-medium rounded-xl hover:shadow-[0_0_20px_rgba(99,102,241,0.3)] transition-all duration-300 cursor-pointer active:scale-95"
          @click="$emit('retry')"
        >
          重试
        </button>
      </div>
    </div>

    <!-- Footer hint -->
    <p class="absolute bottom-6 text-text-muted text-[11px]">
      {{ status === 'idle' || status === 'checking' ? '正在加载...' : status === 'error' ? '' : '首次启动，正在为您准备环境' }}
    </p>
  </div>
</template>
