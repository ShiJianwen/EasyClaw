import { describe, it, expect, vi, beforeEach } from "vitest";
import { mount, flushPromises } from "@vue/test-utils";
import { createRouter, createMemoryHistory } from "vue-router";
import { ref } from "vue";
import ChatView from "../../src/components/ChatView.vue";

const mockSendMessage = vi.fn();
const mockAbortResponse = vi.fn();
const mockClearMessages = vi.fn();
const mockMessages = ref<Array<{
  id: string;
  role: string;
  content: string;
  timestamp: number;
  status: string;
}>>([]);
const mockIsLoading = ref(false);
const mockError = ref<string | null>(null);

vi.mock("../../src/composables/useChat", () => ({
  useChat: () => ({
    messages: mockMessages,
    isLoading: mockIsLoading,
    error: mockError,
    sendMessage: mockSendMessage,
    abortResponse: mockAbortResponse,
    clearMessages: mockClearMessages,
  }),
}));

function createMockRouter() {
  return createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: "/", redirect: "/home" },
      { path: "/home", component: { template: "<div>Home</div>" } },
      { path: "/chat", component: ChatView },
    ],
  });
}

describe("ChatView", () => {
  beforeEach(() => {
    mockMessages.value = [];
    mockIsLoading.value = false;
    mockError.value = null;
    mockSendMessage.mockReset();
    mockAbortResponse.mockReset();
    mockClearMessages.mockReset();
  });

  it("renders empty state when no messages", async () => {
    const router = createMockRouter();
    await router.push("/chat");
    await router.isReady();
    const wrapper = mount(ChatView, {
      global: { plugins: [router] },
    });
    expect(wrapper.find("[data-testid='empty-state']").exists()).toBe(true);
    expect(wrapper.text()).toContain("有什么可以帮您");
  });

  it("renders the chat header with title and clear button", async () => {
    const router = createMockRouter();
    await router.push("/chat");
    await router.isReady();
    const wrapper = mount(ChatView, {
      global: { plugins: [router] },
    });
    expect(wrapper.find("[data-testid='clear-button']").exists()).toBe(true);
    expect(wrapper.text()).toContain("对话助手");
  });

  it("renders user and assistant messages", async () => {
    mockMessages.value = [
      { id: "1", role: "user", content: "Hello", timestamp: Date.now(), status: "done" },
      { id: "2", role: "assistant", content: "Hi there!", timestamp: Date.now(), status: "done" },
    ];
    const router = createMockRouter();
    await router.push("/chat");
    await router.isReady();
    const wrapper = mount(ChatView, {
      global: { plugins: [router] },
    });
    expect(wrapper.find("[data-testid='empty-state']").exists()).toBe(false);
    const userBubbles = wrapper.findAll("[data-testid='message-user']");
    const aiBubbles = wrapper.findAll("[data-testid='message-assistant']");
    expect(userBubbles.length).toBe(1);
    expect(aiBubbles.length).toBe(1);
    expect(userBubbles[0].text()).toContain("Hello");
    expect(aiBubbles[0].text()).toContain("Hi there!");
  });

  it("has an input field and send button", async () => {
    const router = createMockRouter();
    await router.push("/chat");
    await router.isReady();
    const wrapper = mount(ChatView, {
      global: { plugins: [router] },
    });
    expect(wrapper.find("[data-testid='chat-input']").exists()).toBe(true);
    expect(wrapper.find("[data-testid='send-button']").exists()).toBe(true);
  });

  it("calls sendMessage when clicking send button", async () => {
    mockSendMessage.mockResolvedValue(undefined);
    const router = createMockRouter();
    await router.push("/chat");
    await router.isReady();
    const wrapper = mount(ChatView, {
      global: { plugins: [router] },
    });
    const input = wrapper.find("[data-testid='chat-input']");
    await input.setValue("Test message");
    await wrapper.find("[data-testid='send-button']").trigger("click");
    expect(mockSendMessage).toHaveBeenCalledWith("Test message");
  });

  it("shows stop button when loading", async () => {
    mockIsLoading.value = true;
    const router = createMockRouter();
    await router.push("/chat");
    await router.isReady();
    const wrapper = mount(ChatView, {
      global: { plugins: [router] },
    });
    expect(wrapper.find("[data-testid='stop-button']").exists()).toBe(true);
  });

  it("calls abortResponse when clicking stop button", async () => {
    mockIsLoading.value = true;
    const router = createMockRouter();
    await router.push("/chat");
    await router.isReady();
    const wrapper = mount(ChatView, {
      global: { plugins: [router] },
    });
    await wrapper.find("[data-testid='stop-button']").trigger("click");
    expect(mockAbortResponse).toHaveBeenCalled();
  });

  it("shows error message when error exists", async () => {
    mockError.value = "Connection failed";
    const router = createMockRouter();
    await router.push("/chat");
    await router.isReady();
    const wrapper = mount(ChatView, {
      global: { plugins: [router] },
    });
    expect(wrapper.find("[data-testid='error-bar']").exists()).toBe(true);
    expect(wrapper.text()).toContain("Connection failed");
  });
});
