import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export type InitStatus = "idle" | "checking" | "initializing" | "starting_gateway" | "success" | "error";

export interface InitState {
  status: InitStatus;
  progress: number;
  message: string;
  error: string | null;
}

export function useInitialization() {
  const state = ref<InitState>({
    status: "idle",
    progress: 0,
    message: "",
    error: null,
  });

  async function checkAndInitialize() {
    state.value = {
      status: "checking",
      progress: 10,
      message: "正在检查初始化状态...",
      error: null,
    };

    const isInitialized = await invoke<boolean>("check_initialized").catch(
      (err) => {
        console.error("check_initialized failed:", err);
        return false;
      }
    );

    if (isInitialized) {
      state.value = {
        status: "success",
        progress: 100,
        message: "就绪",
        error: null,
      };
      return;
    }

    // Run initialization
    state.value = {
      status: "initializing",
      progress: 30,
      message: "正在初始化配置文件...",
      error: null,
    };

    const initResult = await invoke<string>("initialize_zeroclaw").catch(
      (err) => {
        state.value = {
          status: "error",
          progress: 30,
          message: "初始化失败",
          error: String(err),
        };
        return null;
      }
    );

    if (initResult === null) return;

    // Start gateway (non-blocking - failure is acceptable)
    state.value = {
      status: "starting_gateway",
      progress: 80,
      message: "正在启动服务...",
      error: null,
    };

    await invoke<string>("start_gateway").catch((err) => {
      console.error("Gateway startup failed (non-critical):", err);
    });

    state.value = {
      status: "success",
      progress: 100,
      message: "初始化完成",
      error: null,
    };
  }

  async function retry() {
    await checkAndInitialize();
  }

  return {
    state,
    checkAndInitialize,
    retry,
  };
}
