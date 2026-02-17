# Services Index

Services live under `FluidVoice-1.5.5/Sources/Fluid/Services/`. Each service is documented in its own file; this index lists all and their roles.

| Service | File | Purpose |
|---------|------|--------|
| **AppServices** | (see [02-asr-service] — ASR owned here) | Central container; lazy ASR + AudioHardwareObserver; startup gate `signalUIReady()` / `initializeServicesIfNeeded()`. |
| **ASRService** | ASRService.swift | Full dictation ASR pipeline: audio capture, model selection (FluidAudio/Apple/Whisper), transcription, partial/final text, download/ready state. |
| **TranscriptionProvider** (protocol) | TranscriptionProvider.swift | Abstraction for ASR backends; see [03-transcription-providers.md](./03-transcription-providers.md). |
| **FluidAudioProvider** | FluidAudioProvider.swift | TranscriptionProvider for Parakeet TDT (Apple Silicon); `#if arch(arm64)`. |
| **AppleSpeechProvider** | AppleSpeechProvider.swift | TranscriptionProvider using SFSpeechRecognizer (macOS Speech). |
| **AppleSpeechAnalyzerProvider** | AppleSpeechAnalyzerProvider.swift | TranscriptionProvider using Speech Analyzer API (macOS 26+). |
| **WhisperProvider** | WhisperProvider.swift | TranscriptionProvider using SwiftWhisper. |
| **GlobalHotkeyManager** | GlobalHotkeyManager.swift | CGEvent tap; fires callbacks for dictation, command mode, rewrite mode hotkeys; press-and-hold vs tap. |
| **TypingService** | TypingService.swift | Types text into focused app (CGEvent unicode or clipboard Cmd+V); uses accessibility for focus PID. |
| **TextSelectionService** | TextSelectionService.swift | Returns selected text in focused app via AX API (`kAXSelectedTextAttribute`). |
| **ActiveAppMonitor** | ActiveAppMonitor.swift | Tracks frontmost app (NSWorkspace); publishes activeApp, activeAppIcon, bundleID, name. |
| **NotchOverlayManager** | NotchOverlayManager.swift | Shows/hides DynamicNotch overlay (dictation, command output, rewrite); escape monitors; callbacks for dismiss/follow-up. |
| **CommandModeService** | CommandModeService.swift | Command-mode agent: conversation, LLM + tool calls (MCP), TerminalService execution; chat persistence via ChatHistoryStore. |
| **RewriteModeService** | RewriteModeService.swift | Rewrite/write mode: selected text + prompt or write-from-voice; LLM; TypingService to insert result. |
| **MenuBarManager** | MenuBarManager.swift | NSStatusItem menu bar icon and menu; navigation to sidebar items; configure with ASR for “Start/Stop” etc. |
| **SimpleUpdater** | SimpleUpdater.swift | GitHub release check/download; replace app and relaunch (manual and automatic). |
| **LLMClient** | LLMClient.swift | Unified HTTP/SSE LLM layer; streaming; thinking token extraction; tool-call parsing. (See [03-networking](../03-networking/).) |
| **ModelRepository** | ModelRepository.swift | Default models and base URLs per provider ID. (See [03-networking](../03-networking/).) |
| **AudioDeviceService** / **AudioHardwareObserver** | AudioDeviceService.swift | Input/output device list; preferred device UIDs; system device change observation. |
| **AudioStartupGate** | AudioStartupGate.swift | Gate to avoid starting audio before UI is ready. |
| **ClipboardService** | ClipboardService.swift | Copy string to clipboard; read string from clipboard (NSPasteboard). |
| **MediaPlaybackService** | MediaPlaybackService.swift | Wraps MediaRemoteAdapter for pausing/resuming media during transcription. |
| **MeetingTranscriptionService** | MeetingTranscriptionService.swift | Long-form meeting transcription (separate from main ASR flow). |
| **TranscriptionSoundPlayer** | TranscriptionSoundPlayer.swift | Plays start/stop sounds for transcription (optional). |
| **DictationAIPostProcessingGate** | DictationAIPostProcessingGate.swift | Static gate: whether AI dictation is configured (provider + API key or local). |
| **DictationPromptTestCoordinator** | DictationPromptTestCoordinator.swift | Coordinates testing of dictation prompt (playground). |
| **DebugLogger** | DebugLogger.swift | Central debug logging; respects `SettingsStore.enableDebugLogs`. |
| **FileLogger** | FileLogger.swift | File-based logging if enabled. |
| **ThinkingParsers** | ThinkingParsers.swift | Parses `<think>...</think>` from LLM streams. |
| **ThreadSafeAudioBuffer** | ThreadSafeAudioBuffer.swift | Thread-safe buffer for audio samples. |
| **TerminalService** | TerminalService.swift | Runs shell commands (command mode tool execution). |

## Layering

- **UI/ContentView** creates or gets: `AppServices`, `GlobalHotkeyManager` (after ASR init), `CommandModeService`, `RewriteModeService`, `MenuBarManager`; uses `NotchOverlayManager.shared`, `ActiveAppMonitor.shared`, `TypingService()`, `TextSelectionService.shared`.
- **Services** use: `SettingsStore`, `KeychainService` (via Settings), `LLMClient`, `ModelRepository`, `ChatHistoryStore`, `TranscriptionHistoryStore`, and each other as listed in individual docs.
