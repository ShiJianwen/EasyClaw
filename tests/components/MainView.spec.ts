import { describe, it, expect, vi } from "vitest";
import { mount } from "@vue/test-utils";
import MainView from "../../src/components/MainView.vue";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockResolvedValue(false),
}));

describe("MainView", () => {
  it("renders the navigation bar with brand name", () => {
    const wrapper = mount(MainView);
    const nav = wrapper.find("[data-testid='navbar']");
    expect(nav.exists()).toBe(true);
    expect(nav.text()).toContain("EasyClaw");
  });

  it("renders the welcome banner", () => {
    const wrapper = mount(MainView);
    const banner = wrapper.find("[data-testid='welcome-banner']");
    expect(banner.exists()).toBe(true);
    expect(banner.text()).toContain("欢迎使用 EasyClaw");
  });

  it("renders feature cards", () => {
    const wrapper = mount(MainView);
    const cards = wrapper.findAll("[data-testid='feature-card']");
    expect(cards.length).toBe(4);
  });

  it("renders the status bar", () => {
    const wrapper = mount(MainView);
    const statusBar = wrapper.find("[data-testid='status-bar']");
    expect(statusBar.exists()).toBe(true);
  });

  it("shows version info in status bar", () => {
    const wrapper = mount(MainView);
    const statusBar = wrapper.find("[data-testid='status-bar']");
    expect(statusBar.text()).toContain("v0.1.0");
  });
});
