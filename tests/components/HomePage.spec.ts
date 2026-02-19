import { describe, it, expect, vi } from "vitest";
import { mount, flushPromises } from "@vue/test-utils";
import { createRouter, createMemoryHistory } from "vue-router";
import HomePage from "../../src/components/HomePage.vue";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockResolvedValue(true),
}));

function createMockRouter() {
  return createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: "/", redirect: "/home" },
      { path: "/home", component: HomePage },
      { path: "/chat", component: { template: "<div>Chat</div>" } },
    ],
  });
}

describe("HomePage", () => {
  it("renders the welcome banner", () => {
    const router = createMockRouter();
    const wrapper = mount(HomePage, {
      global: { plugins: [router] },
    });
    const banner = wrapper.find("[data-testid='welcome-banner']");
    expect(banner.exists()).toBe(true);
    expect(banner.text()).toContain("欢迎使用 EasyClaw");
  });

  it("renders 4 feature cards", () => {
    const router = createMockRouter();
    const wrapper = mount(HomePage, {
      global: { plugins: [router] },
    });
    const cards = wrapper.findAll("[data-testid='feature-card']");
    expect(cards.length).toBe(4);
  });

  it("navigates to /chat when clicking chat card", async () => {
    const router = createMockRouter();
    await router.push("/");
    await router.isReady();
    const wrapper = mount(HomePage, {
      global: { plugins: [router] },
    });
    const cards = wrapper.findAll("[data-testid='feature-card']");
    await cards[0].trigger("click");
    await flushPromises();
    expect(router.currentRoute.value.path).toBe("/chat");
  });
});
