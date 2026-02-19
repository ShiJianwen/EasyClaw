import { describe, it, expect, vi } from "vitest";
import { mount } from "@vue/test-utils";
import { createRouter, createMemoryHistory } from "vue-router";
import AppLayout from "../../src/components/AppLayout.vue";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockResolvedValue(false),
}));

function createMockRouter() {
  return createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: "/", redirect: "/home" },
      { path: "/home", component: { template: "<div>Home</div>" } },
      { path: "/chat", component: { template: "<div>Chat</div>" } },
    ],
  });
}

describe("AppLayout", () => {
  it("renders the sidebar with brand icon", () => {
    const router = createMockRouter();
    const wrapper = mount(AppLayout, {
      global: { plugins: [router] },
    });
    const sidebar = wrapper.find("[data-testid='sidebar']");
    expect(sidebar.exists()).toBe(true);
  });

  it("renders sidebar menu items", () => {
    const router = createMockRouter();
    const wrapper = mount(AppLayout, {
      global: { plugins: [router] },
    });
    const menuItems = wrapper.findAll("[data-testid='sidebar-menu-item']");
    expect(menuItems.length).toBeGreaterThanOrEqual(2);
  });

  it("renders the status bar", () => {
    const router = createMockRouter();
    const wrapper = mount(AppLayout, {
      global: { plugins: [router] },
    });
    const statusBar = wrapper.find("[data-testid='status-bar']");
    expect(statusBar.exists()).toBe(true);
  });

  it("shows version info in status bar", () => {
    const router = createMockRouter();
    const wrapper = mount(AppLayout, {
      global: { plugins: [router] },
    });
    const statusBar = wrapper.find("[data-testid='status-bar']");
    expect(statusBar.text()).toContain("v0.1.0");
  });

  it("renders router-view in main content area", () => {
    const router = createMockRouter();
    const wrapper = mount(AppLayout, {
      global: { plugins: [router] },
    });
    expect(wrapper.find("[data-testid='main-content']").exists()).toBe(true);
  });

  it("uses left-right layout (sidebar + content)", () => {
    const router = createMockRouter();
    const wrapper = mount(AppLayout, {
      global: { plugins: [router] },
    });
    const root = wrapper.find("[data-testid='app-layout']");
    expect(root.exists()).toBe(true);
    expect(root.classes()).toContain("flex");
    expect(root.classes()).toContain("flex-row");
  });
});
