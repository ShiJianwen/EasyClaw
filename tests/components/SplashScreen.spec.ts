import { describe, it, expect, vi } from "vitest";
import { mount } from "@vue/test-utils";
import SplashScreen from "../../src/components/SplashScreen.vue";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockResolvedValue(true),
}));

describe("SplashScreen", () => {
  it("renders the brand name", () => {
    const wrapper = mount(SplashScreen, {
      props: {
        status: "checking",
        progress: 10,
        message: "正在检查初始化状态...",
        error: null,
      },
    });
    expect(wrapper.text()).toContain("EasyClaw");
  });

  it("shows the progress bar", () => {
    const wrapper = mount(SplashScreen, {
      props: {
        status: "initializing",
        progress: 50,
        message: "正在初始化配置文件...",
        error: null,
      },
    });
    const progressBar = wrapper.find("[data-testid='progress-bar']");
    expect(progressBar.exists()).toBe(true);
  });

  it("shows status message", () => {
    const wrapper = mount(SplashScreen, {
      props: {
        status: "initializing",
        progress: 30,
        message: "正在初始化配置文件...",
        error: null,
      },
    });
    expect(wrapper.text()).toContain("正在初始化配置文件...");
  });

  it("shows error state with retry button", () => {
    const wrapper = mount(SplashScreen, {
      props: {
        status: "error",
        progress: 30,
        message: "初始化失败",
        error: "Something went wrong",
      },
    });
    expect(wrapper.text()).toContain("Something went wrong");
    const retryBtn = wrapper.find("[data-testid='retry-button']");
    expect(retryBtn.exists()).toBe(true);
  });

  it("emits retry event when retry button is clicked", async () => {
    const wrapper = mount(SplashScreen, {
      props: {
        status: "error",
        progress: 0,
        message: "初始化失败",
        error: "Error",
      },
    });
    const retryBtn = wrapper.find("[data-testid='retry-button']");
    await retryBtn.trigger("click");
    expect(wrapper.emitted("retry")).toBeTruthy();
  });

  it("hides progress bar on error", () => {
    const wrapper = mount(SplashScreen, {
      props: {
        status: "error",
        progress: 0,
        message: "失败",
        error: "Error occurred",
      },
    });
    const progressBar = wrapper.find("[data-testid='progress-bar']");
    expect(progressBar.exists()).toBe(false);
  });
});
