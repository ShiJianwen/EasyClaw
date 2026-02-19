<script setup lang="ts">
import { useRouter } from "vue-router";
import {
  MessageSquare,
  Layers,
  Brain,
  Settings,
} from "lucide-vue-next";

const router = useRouter();

const features = [
  {
    icon: MessageSquare,
    title: "对话助手",
    description: "与 AI 助手自然对话，获取智能回答和建议",
    route: "/chat",
  },
  {
    icon: Layers,
    title: "技能管理",
    description: "管理和配置已安装的技能包，扩展助手能力",
    route: null,
  },
  {
    icon: Brain,
    title: "记忆管理",
    description: "查看和管理 AI 助手的长期记忆与用户画像",
    route: null,
  },
  {
    icon: Settings,
    title: "系统设置",
    description: "自定义模型、网关端口等核心配置参数",
    route: null,
  },
];

function handleCardClick(route: string | null) {
  if (route) {
    router.push(route);
  }
}
</script>

<template>
  <div class="flex-1 px-6 overflow-y-auto">
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
        @click="handleCardClick(feature.route)"
      >
        <div
          class="w-10 h-10 rounded-xl bg-gradient-to-br from-primary/20 to-primary-light/10 flex items-center justify-center mb-4 group-hover:from-primary/30 group-hover:to-primary-light/20 transition-all duration-300"
        >
          <component
            :is="feature.icon"
            class="w-5 h-5 text-primary-lighter"
          />
        </div>
        <h3 class="text-text-primary font-semibold text-sm mb-1.5">
          {{ feature.title }}
        </h3>
        <p class="text-text-muted text-xs leading-relaxed">
          {{ feature.description }}
        </p>
      </div>
    </div>
  </div>
</template>
