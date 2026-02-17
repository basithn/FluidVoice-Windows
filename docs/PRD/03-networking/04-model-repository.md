# Model Repository

## Purpose

Single source of truth for **default** model names and base URLs per AI provider ID. Used by settings UI, LLM call sites, and provider pickers. Does not store user-added models (those come from SettingsStore / savedProviders).

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Services/ModelRepository.swift`

## Depends on

- `Foundation`

## Consumed by

- SettingsStore (default base URL when no saved provider); AISettingsView and provider/model pickers; DictationAIPostProcessingGate (baseURL); LLMClient call sites (base URL for provider).

## Contract

### Constants

- `builtInProviderIDs: [String]` — fluid-1, openai, anthropic, xai, groq, cerebras, google, openrouter, ollama, lmstudio, apple-intelligence.

### Methods

- `defaultModels(for providerID: String) -> [String]` — e.g. openai → ["gpt-4.1"], groq → ["openai/gpt-oss-120b"], ollama/lmstudio → [].
- `defaultBaseURL(for providerID: String) -> String` — e.g. openai → "https://api.openai.com/v1", ollama → "http://localhost:11434/v1", lmstudio → "http://localhost:1234/v1".
- `displayName(for providerID: String) -> String` — human-readable name.
- `isBuiltIn(_ providerID: String) -> Bool`.
- `providerWebsiteURL(for providerID: String) -> (url: String, label: String)?` — e.g. "Get API Key" link for OpenAI console.

## Invariants

- Custom providers (saved in SettingsStore) have no entry in ModelRepository; base URL and models come from SavedProvider.
- Do not add API keys or user data here; read-only metadata only.
