import { ref } from "vue";

const GATEWAY_URL = "http://localhost:18789";

export interface ChatMessage {
    id: string;
    role: "user" | "assistant";
    content: string;
    timestamp: number;
    status: "sending" | "streaming" | "done" | "error";
}

function generateId(): string {
    return `${Date.now()}-${Math.random().toString(36).slice(2, 9)}`;
}

function formatNetworkError(err: unknown): string {
    const raw = err instanceof Error ? err.message : String(err);
    return `无法连接到服务，请检查 Gateway 是否已启动。\n原始错误：${raw}`;
}

function formatHttpError(status: number, serverMsg?: string): string {
    if (serverMsg) return serverMsg;
    const statusMap: Record<number, string> = {
        400: `请求参数错误 (${status})`,
        401: `认证失败，请检查配置 (${status})`,
        403: `访问被拒绝 (${status})`,
        404: `接口不存在，请检查 Gateway 版本 (${status})`,
        429: `请求过于频繁，请稍后重试 (${status})`,
        500: `服务内部错误 (${status})`,
        502: `网关错误 (${status})`,
        503: `服务暂时不可用 (${status})`,
    };
    return statusMap[status] ?? `请求失败 (${status})`;
}

export function useChat() {
    const messages = ref<ChatMessage[]>([]);
    const isLoading = ref(false);
    const error = ref<string | null>(null);
    let abortController: AbortController | null = null;

    function pushErrorAssistant(errorMsg: string): void {
        messages.value.push({
            id: generateId(),
            role: "assistant",
            content: errorMsg,
            timestamp: Date.now(),
            status: "error",
        });
    }

    async function sendMessage(content: string): Promise<void> {
        const trimmed = content.trim();
        if (!trimmed) return;

        error.value = null;

        const userMessage: ChatMessage = {
            id: generateId(),
            role: "user",
            content: trimmed,
            timestamp: Date.now(),
            status: "done",
        };
        messages.value.push(userMessage);
        isLoading.value = true;

        abortController = new AbortController();

        let response: Response;
        try {
            response = await fetch(`${GATEWAY_URL}/webhook`, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    message: trimmed,
                }),
                signal: abortController.signal,
            });
        } catch (err: unknown) {
            if (err instanceof DOMException && err.name === "AbortError") {
                isLoading.value = false;
                return;
            }
            const errorMsg = formatNetworkError(err);
            error.value = errorMsg;
            pushErrorAssistant(errorMsg);
            isLoading.value = false;
            return;
        }

        if (!response.ok) {
            let serverMsg: string | undefined;
            try {
                const body = await response.json();
                if (body.error) {
                    serverMsg = body.error;
                }
            } catch {
                // Use default error message
            }
            const errorMsg = formatHttpError(response.status, serverMsg);
            error.value = errorMsg;
            pushErrorAssistant(errorMsg);
            isLoading.value = false;
            return;
        }

        const assistantMessage: ChatMessage = {
            id: generateId(),
            role: "assistant",
            content: "",
            timestamp: Date.now(),
            status: "streaming",
        };
        messages.value.push(assistantMessage);

        try {
            const body = await response.json();
            assistantMessage.content = body.reply ?? body.response ?? body.content ?? JSON.stringify(body);
            assistantMessage.status = "done";
        } catch (err: unknown) {
            if (err instanceof DOMException && err.name === "AbortError") {
                assistantMessage.status = "done";
            } else {
                const errorMsg = err instanceof Error ? err.message : String(err);
                error.value = errorMsg;
                assistantMessage.status = "error";
                assistantMessage.content = errorMsg;
            }
        }

        isLoading.value = false;
    }

    function abortResponse(): void {
        if (abortController) {
            abortController.abort();
            abortController = null;
        }
        // Mark any streaming messages as done
        for (const msg of messages.value) {
            if (msg.status === "streaming") {
                msg.status = "done";
            }
        }
        isLoading.value = false;
    }

    function clearMessages(): void {
        messages.value = [];
        error.value = null;
    }

    return {
        messages,
        isLoading,
        error,
        sendMessage,
        abortResponse,
        clearMessages,
    };
}
