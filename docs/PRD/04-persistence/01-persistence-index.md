# Persistence Index

Persistence covers user settings (UserDefaults), secure API key storage (Keychain), transcription history, and command-mode chat history. All under `FluidVoice-1.5.5/Sources/Fluid/Persistence/`.

| Component | File | Purpose |
|-----------|------|--------|
| **SettingsStore** | SettingsStore.swift | Single ObservableObject for all app settings; UserDefaults keys in private enum Keys; API keys read/write via KeychainService. |
| **KeychainService** | KeychainService.swift | Store/fetch/delete provider API keys as generic password; service name and account constant; migration from legacy keys. |
| **TranscriptionHistoryStore** | TranscriptionHistoryStore.swift | Append-only list of TranscriptionHistoryEntry (raw, processed, app, window, timestamp); UserDefaults; max entries; selectedEntryID. |
| **ChatHistoryStore** | ChatHistoryStore.swift | Command-mode chat sessions (id, title, messages, createdAt, updatedAt); UserDefaults; current session; recent list; max chats. |

## Data flow

- **Settings:** UI binds to SettingsStore properties; getters/setters read/write UserDefaults (and Keychain for API keys). Keys are internal; see [02-settings-store.md](./02-settings-store.md) for key list.
- **API keys:** SettingsStore.getAPIKey(for:) / setAPIKey(for:key:) delegate to KeychainService; keys stored per provider ID in a single Keychain item (JSON or similar).
- **Transcription history:** ASRService (or delivery path) calls TranscriptionHistoryStore.addEntry(...) after each transcription delivery; history view reads entries and selectedEntryID.
- **Chat history:** CommandModeService saves/loads current session and list via ChatHistoryStore; createNewChat, switchToChat, clearCurrentChat.

## Dependencies

- Persistence does not depend on Services or UI (except SettingsStore is ObservableObject for SwiftUI).
- SettingsStore and KeychainService are used by almost every module; TranscriptionHistoryStore and ChatHistoryStore by ASR flow and CommandModeService respectively.
