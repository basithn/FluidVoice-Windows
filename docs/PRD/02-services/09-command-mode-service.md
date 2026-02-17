# Command Mode Service

## Purpose

Runs the “command mode” agent: user speaks a command; service sends messages to the LLM with MCP tools (e.g. run_command, open_app); LLM may return tool calls that are executed (e.g. via TerminalService); results are streamed and shown in the command output notch. Persists chat sessions in ChatHistoryStore.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Services/CommandModeService.swift`
- Tools/LLM: `FluidVoice-1.5.5/Sources/Fluid/Networking/FunctionCallingProvider.swift`, `LLMClient.swift`
- Execution: `FluidVoice-1.5.5/Sources/Fluid/Services/TerminalService.swift`
- Persistence: `FluidVoice-1.5.5/Sources/Fluid/Persistence/ChatHistoryStore.swift`

## Depends on

- `Combine`, `Foundation`
- `LLMClient.shared`, `ChatHistoryStore.shared`, `TerminalService`, `NotchContentState`, `SettingsStore` (provider, model, API key, confirm-before-execute)
- `FunctionCallingProvider` for tool definitions and request/response shapes
- `ThinkingParsers` for <think> extraction

## Consumed by

- ContentView (holds `@StateObject` CommandModeService); CommandModeView; NotchOverlayManager (command output content); GlobalHotkeyManager (command shortcut callback).

## Contract

### State (published)

- `conversationHistory: [Message]` — user, assistant, tool messages with optional thinking and toolCall.
- `isProcessing: Bool`
- `pendingCommand: PendingCommand?` — command awaiting confirmation (if confirm-before-execute).
- `currentStep: AgentStep?` — thinking | checking | executing | verifying | completed(success).
- `streamingText`, `streamingThinkingText` — real-time UI.
- `currentChatID: String?` — current ChatHistoryStore session id.

### Models

- `Message(role, content, thinking, toolCall, stepType, timestamp)`; roles: user, assistant, tool.
- `PendingCommand(id, command, workingDirectory, purpose)`.
- `AgentStep`: thinking(String), checking(String), executing(String), verifying(String), completed(Bool).

### Key methods

- `processUserInput(_ text: String)` — append user message; call LLM with tools; handle tool calls (run_command → TerminalService, etc.); append assistant/tool messages; update streaming and step.
- `confirmPendingCommand()` / `rejectPendingCommand()` — when confirm-before-execute is on.
- `createNewChat()` — save current, create new session, clear conversation and notch state.
- `switchToChat(id:)` — load session by id, set currentChatID, sync to notch.
- `clearHistory()` — clear in-memory and store.
- `getRecentChats() -> [ChatSession]` — for dropdown.

### Tool execution

- run_command: `TerminalService` runs shell command; stdout/stderr returned as tool result.
- open_app / other tools: as defined in FunctionCallingProvider; parameters from LLM arguments.
- Max turns (e.g. 20) to prevent infinite loops; optional confirm before first execution per user setting.

### Persistence

- On new chat or switch: `ChatHistoryStore.createNewChat()` or load by id.
- After assistant/tool messages: save current session (messages, title from first user message).
- `ChatHistoryStore` holds last N sessions (e.g. 30); oldest dropped.

## Invariants

- All UI updates (conversationHistory, streaming, step) on MainActor.
- Do not create new chat or switch while `isProcessing` is true.
- Notch state (`NotchContentState.shared`) cleared or updated when chat changes.

## Edge cases

- LLM returns invalid tool args: log and show error in conversation.
- Terminal command fails: tool result contains stderr; step may be `.completed(false)`.
- User revokes accessibility: TerminalService or typing may fail; document for support.
