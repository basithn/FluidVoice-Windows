# Networking Index

Networking covers LLM/AI API calls, model metadata, and model file downloads. All under `FluidVoice-1.5.5/Sources/Fluid/Networking/`.

| Component | File | Purpose |
|-----------|------|--------|
| **AIProvider** | AIProvider.swift | Protocol and OpenAI-compatible implementation for simple chat completion (dictation cleanup, non-streaming). |
| **LLMClient** | (Services) LLMClient.swift | Unified streaming LLM layer: HTTP/SSE, <think> extraction, tool-call parsing; used by Command and Rewrite. |
| **ModelRepository** | (Services) ModelRepository.swift | Default model lists and base URLs per provider ID; display names; website URLs. |
| **ModelDownloader** | ModelDownloader.swift | Downloads ASR models (e.g. Whisper, Parakeet) from Hugging Face; CoreML/MLModel loading for FluidAudio path. |
| **FunctionCallingProvider** | FunctionCallingProvider.swift | MCP/OpenAI function-calling: tool definitions (run_command, open_app, etc.), request/response encoding, tool result injection. |
| **AppleIntelligenceProvider** | AppleIntelligenceProvider.swift | Apple Intelligence API bridge for dictation/LLM when providerID is apple-intelligence. |

## Data flow

- **Dictation AI:** SettingsStore (provider, model, API key) → AIProvider or AppleIntelligenceProvider → LLM; response = cleaned text.
- **Command/Rewrite:** SettingsStore + ChatHistoryStore (conversation) → LLMClient with optional tools (FunctionCallingProvider) → streamed content + tool calls → TerminalService or other executor.
- **Model list:** ModelRepository.defaultModels(for:), defaultBaseURL(for:); UI and services read from it.
- **ASR model files:** ModelDownloader used by WhisperProvider and FluidAudio path (ModelRepository + Hugging Face URLs).

## Dependencies

- Networking does not depend on UI. It may use SettingsStore/Keychain for API keys (via callers that pass keys).
- LLMClient and AIProvider are used only by services (ASRService, CommandModeService, RewriteModeService) or by UI that delegates to them.
