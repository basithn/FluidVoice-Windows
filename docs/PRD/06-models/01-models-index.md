# Models Index

Shared data models used across the app (excluding persistence-specific types like ChatMessage/ChatSession, which live in Persistence docs). Primary model under Models/ is HotkeyShortcut.

## Location

- `FluidVoice-1.5.5/Sources/Fluid/Models/HotkeyShortcut.swift`

## Other types

- **ASRTranscriptionResult:** TranscriptionProvider.swift (text, confidence).
- **SpeechModel, DictationPromptProfile, SavedProvider:** SettingsStore.swift (see [04-persistence/02-settings-store.md](../04-persistence/02-settings-store.md)).
- **ChatMessage, ChatSession, PendingCommand, AgentStep:** CommandModeService / ChatHistoryStore.
- **TranscriptionHistoryEntry:** TranscriptionHistoryStore.
- **SidebarItem:** ContentView (enum of sidebar destinations).

For HotkeyShortcut contract and coding details see [02-hotkey-shortcut.md](./02-hotkey-shortcut.md).
