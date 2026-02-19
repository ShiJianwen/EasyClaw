import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { nextTick } from "vue";
import { useChat } from "../../src/composables/useChat";

function createJsonResponse(body: Record<string, unknown>, status = 200) {
  return new Response(JSON.stringify(body), {
    status,
    headers: { "Content-Type": "application/json" },
  });
}

describe("useChat", () => {
  let fetchSpy: ReturnType<typeof vi.spyOn>;

  beforeEach(() => {
    fetchSpy = vi.spyOn(globalThis, "fetch");
  });

  afterEach(() => {
    fetchSpy.mockRestore();
  });

  it("has empty messages and idle state initially", () => {
    const { messages, isLoading, error } = useChat();
    expect(messages.value).toEqual([]);
    expect(isLoading.value).toBe(false);
    expect(error.value).toBeNull();
  });

  it("adds user message and sets loading on sendMessage", async () => {
    fetchSpy.mockResolvedValue(
      createJsonResponse({ reply: "Hi there" })
    );

    const { messages, isLoading, sendMessage } = useChat();
    const promise = sendMessage("Hello");

    await nextTick();
    expect(messages.value.length).toBeGreaterThanOrEqual(1);
    expect(messages.value[0].role).toBe("user");
    expect(messages.value[0].content).toBe("Hello");
    expect(isLoading.value).toBe(true);

    await promise;
  });

  it("sends correct request to /webhook endpoint", async () => {
    fetchSpy.mockResolvedValue(
      createJsonResponse({ reply: "Hello" })
    );

    const { sendMessage } = useChat();
    await sendMessage("Hi");

    expect(fetchSpy).toHaveBeenCalledWith(
      "http://localhost:18789/webhook",
      expect.objectContaining({
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ message: "Hi" }),
      })
    );
  });

  it("parses assistant response from JSON reply field", async () => {
    fetchSpy.mockResolvedValue(
      createJsonResponse({ reply: "Hello World" })
    );

    const { messages, isLoading, sendMessage } = useChat();
    await sendMessage("Hi");

    expect(messages.value.length).toBe(2);
    expect(messages.value[1].role).toBe("assistant");
    expect(messages.value[1].content).toBe("Hello World");
    expect(messages.value[1].status).toBe("done");
    expect(isLoading.value).toBe(false);
  });

  it("adds error assistant message on network failure", async () => {
    fetchSpy.mockRejectedValue(new Error("Failed to fetch"));

    const { messages, error, isLoading, sendMessage } = useChat();
    await sendMessage("Hi");

    expect(error.value).toContain("无法连接到服务");
    expect(error.value).toContain("Failed to fetch");
    expect(isLoading.value).toBe(false);
    // User message + error assistant message
    expect(messages.value.length).toBe(2);
    expect(messages.value[1].role).toBe("assistant");
    expect(messages.value[1].status).toBe("error");
    expect(messages.value[1].content).toContain("Failed to fetch");
  });

  it("adds error assistant message on connection refused", async () => {
    fetchSpy.mockRejectedValue(new TypeError("fetch failed"));

    const { messages, error, sendMessage } = useChat();
    await sendMessage("Hi");

    expect(error.value).toContain("无法连接到服务");
    expect(messages.value.length).toBe(2);
    expect(messages.value[1].role).toBe("assistant");
    expect(messages.value[1].status).toBe("error");
  });

  it("adds error assistant message on HTTP error with body", async () => {
    fetchSpy.mockResolvedValue(
      createJsonResponse({ error: "LLM provider rate limited" }, 429)
    );

    const { messages, error, sendMessage } = useChat();
    await sendMessage("Hi");

    expect(error.value).toBe("LLM provider rate limited");
    expect(messages.value.length).toBe(2);
    expect(messages.value[1].role).toBe("assistant");
    expect(messages.value[1].status).toBe("error");
    expect(messages.value[1].content).toBe("LLM provider rate limited");
  });

  it("adds error assistant message on HTTP error without body", async () => {
    fetchSpy.mockResolvedValue(
      new Response("Internal Server Error", { status: 500 })
    );

    const { messages, error, sendMessage } = useChat();
    await sendMessage("Hi");

    expect(error.value).toContain("500");
    expect(messages.value.length).toBe(2);
    expect(messages.value[1].role).toBe("assistant");
    expect(messages.value[1].status).toBe("error");
    expect(messages.value[1].content).toContain("500");
  });

  it("adds error assistant message on HTTP 401 unauthorized", async () => {
    fetchSpy.mockResolvedValue(
      createJsonResponse({ error: "Unauthorized: pairing required" }, 401)
    );

    const { messages, error, sendMessage } = useChat();
    await sendMessage("Hi");

    expect(error.value).toBe("Unauthorized: pairing required");
    expect(messages.value[1].status).toBe("error");
  });

  it("adds error assistant message on JSON parse failure", async () => {
    fetchSpy.mockResolvedValue(
      new Response("not json", {
        status: 200,
        headers: { "Content-Type": "application/json" },
      })
    );

    const { messages, error, sendMessage } = useChat();
    await sendMessage("Hi");

    expect(messages.value.length).toBe(2);
    expect(messages.value[1].status).toBe("error");
    expect(error.value).toBeTruthy();
  });

  it("can clear all messages", async () => {
    fetchSpy.mockResolvedValue(
      createJsonResponse({ reply: "Reply" })
    );

    const { messages, clearMessages, sendMessage } = useChat();
    await sendMessage("Hello");
    expect(messages.value.length).toBe(2);

    clearMessages();
    expect(messages.value).toEqual([]);
  });

  it("does not send empty messages", async () => {
    const { sendMessage } = useChat();
    await sendMessage("   ");
    expect(fetchSpy).not.toHaveBeenCalled();
  });
});
