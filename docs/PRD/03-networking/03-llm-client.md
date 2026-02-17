# LLM Client

## Purpose

Unified client for all streaming LLM calls: builds HTTP/SSE request from messages and config; parses SSE for content and optional <think>...</think>; parses tool_calls from assistant message; returns structured Response (thinking, content, toolCalls). Used by Command mode and Rewrite mode.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Services/LLMClient.swift` (logically networking; lives under Services)

## Depends on

- `Foundation`, URLSession
- SettingsStore / caller for model, baseURL, apiKey; optional extra params (reasoning_effort, etc.)
- ThinkingParsers (<think> extraction)
- JSON encoding for request; SSE line-by-line parsing for stream

## Consumed by

- CommandModeService, RewriteModeService (and any flow that needs streaming + tools or thinking).

## Contract

### Config

- `messages: [[String: Any]]` — OpenAI-style (role, content; optional tool_calls, tool_call_id)
- `model`, `baseURL`, `apiKey`, `streaming`, `tools` (array of tool definitions), `temperature`, `maxTokens`, `extraParameters`
- Callbacks: `onThinkingStart`, `onThinkingChunk`, `onThinkingEnd`, `onContentChunk`, `onToolCallStart` (optional; for UI)

### Response

- `Response(thinking: String?, content: String, toolCalls: [ToolCall])`
- `ToolCall(id, name, arguments: [String: Any])` with helpers `getString(_)`, `getOptionalString(_)`

### Method

- `func stream(config: Config) async throws -> Response` — sends POST to baseURL + "/chat/completions" (or equivalent); if streaming, reads SSE; extracts <think> blocks into thinking; accumulates content; parses tool_calls from last assistant delta or message; returns Response. Retries per config (maxRetries, retryDelayMs).

### Errors

- `LLMError`: invalidURL, invalidResponse, httpError(code, message), networkError(Error), encodingError.

## Invariants

- Main thread for callbacks if provided; internal HTTP can be async.
- Tool definitions must match FunctionCallingProvider schema when used for command mode.
- thinking is stripped from content in response; content is the user-visible text.
