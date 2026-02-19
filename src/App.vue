<script setup lang="ts">
import { onMounted } from "vue";
import SplashScreen from "./components/SplashScreen.vue";
import MainView from "./components/MainView.vue";
import { useInitialization } from "./composables/useInitialization";

const { state, checkAndInitialize, retry } = useInitialization();

onMounted(() => {
  checkAndInitialize();
});
</script>

<template>
  <div class="w-screen h-screen bg-dark-900">
    <Transition name="fade" mode="out-in">
      <MainView
        v-if="state.status === 'success'"
        key="main"
      />
      <SplashScreen
        v-else
        key="splash"
        :status="state.status"
        :progress="state.progress"
        :message="state.message"
        :error="state.error"
        @retry="retry"
      />
    </Transition>
  </div>
</template>

<style>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.4s ease, transform 0.4s ease;
}

.fade-enter-from {
  opacity: 0;
  transform: translateY(10px);
}

.fade-leave-to {
  opacity: 0;
}

.fade-enter-to,
.fade-leave-from {
  opacity: 1;
  transform: translateY(0);
}
</style>
