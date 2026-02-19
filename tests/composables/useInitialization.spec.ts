import { describe, it, expect, vi, beforeEach } from "vitest";

// Mock @tauri-apps/api/core before importing the composable
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

import { invoke } from "@tauri-apps/api/core";
import { useInitialization } from "../../src/composables/useInitialization";

const mockedInvoke = vi.mocked(invoke);

describe("useInitialization", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should start with checking status", () => {
    const { state } = useInitialization();
    expect(state.value.status).toBe("idle");
    expect(state.value.progress).toBe(0);
    expect(state.value.error).toBeNull();
  });

  it("should go directly to success if already initialized", async () => {
    mockedInvoke.mockResolvedValueOnce(true); // check_initialized returns true

    const { state, checkAndInitialize } = useInitialization();
    await checkAndInitialize();

    expect(mockedInvoke).toHaveBeenCalledWith("check_initialized");
    expect(state.value.status).toBe("success");
    expect(state.value.progress).toBe(100);
  });

  it("should run initialization if not yet initialized", async () => {
    mockedInvoke
      .mockResolvedValueOnce(false) // check_initialized returns false
      .mockResolvedValueOnce("OK") // initialize_zeroclaw
      .mockResolvedValueOnce("OK"); // start_gateway

    const { state, checkAndInitialize } = useInitialization();
    await checkAndInitialize();

    expect(mockedInvoke).toHaveBeenCalledWith("check_initialized");
    expect(mockedInvoke).toHaveBeenCalledWith("initialize_zeroclaw");
    expect(mockedInvoke).toHaveBeenCalledWith("start_gateway");
    expect(state.value.status).toBe("success");
    expect(state.value.progress).toBe(100);
  });

  it("should handle initialization error", async () => {
    mockedInvoke
      .mockResolvedValueOnce(false) // check_initialized
      .mockRejectedValueOnce("Init failed"); // initialize_zeroclaw

    const { state, checkAndInitialize } = useInitialization();
    await checkAndInitialize();

    expect(state.value.status).toBe("error");
    expect(state.value.error).toBe("Init failed");
  });

  it("should handle gateway failure gracefully (not block success)", async () => {
    mockedInvoke
      .mockResolvedValueOnce(false) // check_initialized
      .mockResolvedValueOnce("OK") // initialize_zeroclaw
      .mockRejectedValueOnce("Gateway failed"); // start_gateway fails

    const { state, checkAndInitialize } = useInitialization();
    await checkAndInitialize();

    // Gateway failure should not block success
    expect(state.value.status).toBe("success");
    expect(state.value.progress).toBe(100);
  });

  it("should allow retry after error", async () => {
    mockedInvoke
      .mockResolvedValueOnce(false) // first check
      .mockRejectedValueOnce("fail") // first init
      .mockResolvedValueOnce(false) // second check (retry)
      .mockResolvedValueOnce("OK") // second init
      .mockResolvedValueOnce("OK"); // gateway

    const { state, checkAndInitialize, retry } = useInitialization();

    await checkAndInitialize();
    expect(state.value.status).toBe("error");

    await retry();
    expect(state.value.status).toBe("success");
  });
});
