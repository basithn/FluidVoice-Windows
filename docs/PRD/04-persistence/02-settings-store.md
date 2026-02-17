# Settings Store

## Purpose

Single source of truth for all user preferences: hotkeys, ASR model, AI provider and model, API keys (via Keychain), UI toggles (AI on/off, streaming, sounds, dock, launch at startup), dictation prompts, overlay position, and feature flags. Exposes ObservableObject for SwiftUI; keys are internal.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Persistence/SettingsStore.swift`

## Depends on

- `AppKit`, `ApplicationServices`, `Combine`, `Foundation`, `SwiftUI`; `ServiceManagement` (launch at startup); `#if canImport(FluidAudio) FluidAudio` for migration
- KeychainService.shared for API key read/write
- UserDefaults.standard

## Consumed by

- All modules that need settings: ContentView, ASRService, GlobalHotkeyManager, CommandModeService, RewriteModeService, DictationAIPostProcessingGate, SimpleUpdater/AppDelegate, AnalyticsService, NotchOverlayManager, etc.

## Contract

### Singleton

- `static let shared = SettingsStore()`; `ObservableObject`; init runs migration and scrub for provider keys.

### Key categories (Keys enum)

- **AI:** EnableAIProcessing, AvailableAIModels, AvailableModelsByProvider, SelectedAIModel, SelectedModelByProvider, SelectedProviderID, SavedProviders, ProviderAPIKeys (legacy scrub), VerifiedProviderFingerprints.
- **Audio:** PreferredInputDeviceUID, PreferredOutputDeviceUID, SyncAudioDevicesWithSystem, VisualizerNoiseThreshold.
- **Hotkeys:** HotkeyShortcutKey, CommandModeHotkeyShortcut, RewriteModeHotkeyShortcut, CommandModeShortcutEnabled, RewriteModeShortcutEnabled, PressAndHoldMode.
- **UI/App:** LaunchAtStartup, ShowInDock, AccentColorOption, EnableTranscriptionSounds, TranscriptionStartSound, CopyTranscriptionToClipboard, AutoUpdateCheckEnabled, LastUpdateCheckDate, UpdatePromptSnoozedUntil, SnoozedUpdateVersion, OverlayPosition, OverlayBottomOffset, OverlaySize.
- **Dictation:** EnableStreamingPreview, EnableAIStreaming, CustomDictationPrompt, DictationPromptProfiles, SelectedDictationPromptID, DefaultDictationPromptOverride.
- **ASR:** SelectedSpeechModel (unified), legacy SelectedTranscriptionProvider, WhisperModelSize (migration).
- **Command/Rewrite:** CommandModeSelectedModel, CommandModeSelectedProviderID, CommandModeConfirmBeforeExecute, CommandModeLinkedToGlobal, RewriteModeSelectedModel, RewriteModeSelectedProviderID, RewriteModeLinkedToGlobal, ModelReasoningConfigs, RewriteModeShortcutEnabled, ShowThinkingTokens.
- **Other:** SaveTranscriptionHistory, FillerWords, RemoveFillerWordsEnabled, GAAVModeEnabled, CustomDictionaryEntries, PauseMediaDuringTranscription, ShareAnonymousAnalytics, PlaygroundUsed.

### API key access

- `getAPIKey(for providerID: String) -> String?` — from KeychainService or savedProviders; never from UserDefaults in clear text.
- `setAPIKey(for:key:)` — writes to KeychainService; updates SavedProviders if custom.
- Migration: legacy ProviderAPIKeys moved to Keychain; keys scrubbed from UserDefaults.

### Types

- **SpeechModel:** enum (parakeetTDT, parakeetTDTV2, appleSpeech, appleSpeechAnalyzer, whisperTiny, whisperBase, whisperSmall, …); rawValue stored; availableModels / defaultModel by architecture.
- **DictationPromptProfile:** id, name, prompt, createdAt, updatedAt; array in UserDefaults.
- **SavedProvider:** id, name, baseURL, apiKey (or key reference), model list; for custom providers.
- **AccentColorOption:** enum for accent color.
- **HotkeyShortcut:** Codable; stored as keyCode + modifierFlagsRawValue for HotkeyShortcutKey, CommandModeHotkeyShortcut, RewriteModeHotkeyShortcut.

### Static helpers

- `SettingsStore.baseDictationPromptText()` — fixed role/intent text (not user-editable).
- `SettingsStore.defaultDictationPromptBodyText()` — default body for "Default" profile.
- Update snooze: `shouldCheckForUpdates()`, `updateLastCheckDate()`, `snoozeUpdatePrompt(forVersion:)`, `shouldShowUpdatePrompt(forVersion:)`, `clearUpdateSnooze()`.

## Invariants

- Never persist API keys in UserDefaults; use KeychainService only. Keys enum values are internal; do not expose raw keys to UI strings.
- selectedSpeechModel migration: if Keys.selectedSpeechModel is missing, migrate from SelectedTranscriptionProvider + WhisperModelSize to a single SpeechModel value and write it back.

## Edge cases

- First launch: many keys missing; getters return sensible defaults (e.g. default hotkey, OpenAI provider, Parakeet or Whisper by arch).
- Keychain access denied: getAPIKey returns nil; UI may show Keychain permission alert (see AISettings).
