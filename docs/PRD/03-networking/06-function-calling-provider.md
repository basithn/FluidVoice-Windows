# Function Calling Provider (MCP Tools)

## Purpose

Defines OpenAI-compatible function/tool definitions for Command mode: run_command, open_app, etc. Encodes chat request with tools and tool_choice; parses assistant message for tool_calls; formats tool results for next message. Enables the LLM to request shell execution or app launch; results are fed back as tool role messages.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Networking/FunctionCallingProvider.swift`

## Depends on

- `Foundation`
- LLMClient (or same HTTP layer) to send request with tools; TerminalService for run_command; NSWorkspace/AppleScript for open_app if applicable

## Consumed by

- CommandModeService: builds tool list; sends messages + tools to LLM; when response contains tool_calls, executes (e.g. run_command via TerminalService), then appends tool result and continues turn.

## Contract

### Tool definitions (schema)

- Each tool: name, description, parameters (JSON schema). Example: run_command with command, working_directory, purpose; open_app with app name or bundle id.
- tool_choice: "auto" or "required" (implementation-specific).

### Request encoding

- ChatRequest: model, messages (with optional tool_calls, tool_call_id for assistant/tool), temperature, tools, tool_choice.
- ChatMessage: role (user/assistant/tool), content, tool_calls, tool_call_id, name (for tool role).

### Response parsing

- ChatResponse.choices[0].message may contain tool_calls: array of { id, type, function: { name, arguments } }.
- arguments is JSON string; parse to [String: Any] or ToolCall type for execution.

### Execution

- run_command: TerminalService runs command (and optional working directory); stdout/stderr (or combined) returned as tool result content.
- open_app: resolve app by name or bundle ID; NSWorkspace.shared.openApplication or open URL.
- Other tools: extend schema and execution in CommandModeService or dedicated executor.

## Invariants

- Tool names and argument keys must match what the LLM is instructed to use (system prompt for command mode).
- Tool result message: role "tool", tool_call_id set, content = string result (or error message).
- Max turns and confirm-before-execute are enforced in CommandModeService, not here.
