# Glossary

Terms used consistently across the PRD and codebase.

| Term | Definition |
|------|------------|
| **ASR** | Automatic Speech Recognition. The component that converts audio to text (e.g. Parakeet, Apple Speech, Whisper). |
| **Dictation** | The primary flow: record voice → transcribe → optional AI cleanup → deliver (type or clipboard). |
| **Command mode** | Voice-driven agent: user speaks a command; LLM may use MCP tools (e.g. run terminal command, open app); result shown in overlay or executed. |
| **Rewrite mode** | User selects text and gives a voice instruction; LLM rewrites the selection; app can type the result back. |
| **Write mode** | No selection; user describes what to write; LLM generates text; app types it (subflow of rewrite flow). |
| **Transcription provider** | Implementation of `TranscriptionProvider` protocol: FluidAudio, AppleSpeech, AppleSpeechAnalyzer, Whisper. |
| **Speech model** | User-facing choice in Settings: `SettingsStore.SpeechModel` (e.g. Parakeet TDT v3, Whisper base, Apple Speech). Maps to a transcription provider + model size where applicable. |
| **AI provider** | LLM backend for dictation cleanup / command / rewrite: OpenAI, Anthropic, Groq, OpenRouter, Ollama, LM Studio, Apple Intelligence, etc. Identified by `providerID` (e.g. `"openai"`, `"ollama"`). |
| **Notch** | The DynamicNotchKit-based floating overlay (compact + expanded) used for dictation preview, command output, rewrite result. |
| **Bottom overlay** | Separate overlay at bottom of screen (e.g. for meeting transcription or alternate UX). |
| **Global hotkey** | System-wide shortcut that works when any app is focused; implemented via CGEvent tap; requires Accessibility. |
| **Type-into-app** | Sending transcribed (or AI) text to the currently focused app as if the user typed it (CGEvent unicode or clipboard Cmd+V fallback). |
| **MCP** | Model Context Protocol. Used in command mode: LLM can call tools (e.g. run_command, open_app) defined in `FunctionCallingProvider`. |
| **Keychain** | macOS Keychain; used by `KeychainService` to store provider API keys. |
| **SettingsStore** | Singleton `ObservableObject` holding all user preferences (UserDefaults + Keychain-backed API keys). |
| **AppServices** | Singleton container that owns lazy `ASRService` and `AudioHardwareObserver`; used to avoid heavy init before UI is ready. |
| **FluidAudio** | Third-party package (Parakeet TDT CoreML); Apple Silicon only; used via `FluidAudioProvider`. |
| **GAAV mode** | Setting that removes capitalization and trailing punctuation from transcription output (e.g. for captions). |
| **Filler words** | User-configurable list of words to strip from transcription (e.g. "um", "uh"); `removeFillerWordsEnabled` toggles. |
| **Dictation prompt** | System prompt sent to the LLM for dictation cleanup. Built-in default + optional override + per-profile prompts in `SettingsStore`. |
