# Chat History Store

## Purpose

Stores Command-mode chat sessions: each session has id, title, messages (user/assistant/tool with optional toolCall and stepType), createdAt, updatedAt. Maintains a "current" session and a bounded list of recent sessions for the dropdown. Persisted in UserDefaults.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Persistence/ChatHistoryStore.swift`

## Depends on

- `Combine`, `Foundation`
- UserDefaults; Keys.chatSessions (and possibly currentChatID key)

## Consumed by

- CommandModeService: loadCurrentChatFromStore, saveCurrentChat, createNewChat, getRecentChats, switchToChat, clearCurrentChat.

## Contract

### Models

- **ChatMessage:** id, role (user/assistant/tool), content, toolCall (id, command, workingDirectory, purpose), stepType (normal, thinking, checking, executing, verifying, success, failure), timestamp. Codable.
- **ChatSession:** id (String UUID), title, createdAt, updatedAt, messages: [ChatMessage]. Codable, Identifiable. updateTitleFromFirstMessage(); relativeTimeString for display.

### Store API

- `static let shared`; `@MainActor`; `ObservableObject`.
- `var currentSession: ChatSession?` — the active chat (by currentChatID).
- `func createNewChat() -> ChatSession` — new session; append to list; set as current; trim to max (e.g. 30); persist.
- `func getRecentChats(excludingCurrent: Bool) -> [ChatSession]` — ordered by updatedAt; optional exclude current.
- `func switchToChat(id: String)` — set currentChatID; currentSession becomes that session; persist.
- `func clearCurrentChat()` — clear messages of current session (or remove session); persist.
- Save: after each message or title change, persist current session and list.

### Persistence

- Keys: chatSessions (array of sessions), currentChatID (string). Encode sessions as JSON.
- Max chats: e.g. 30; when over, drop oldest by updatedAt.

## Invariants

- CommandModeService must save after appending assistant/tool messages so state survives restart.
- Title: when first user message is added, updateTitleFromFirstMessage() truncates to ~50 chars for display.
