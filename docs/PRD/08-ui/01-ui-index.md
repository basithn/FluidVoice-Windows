# UI Index

UI covers the main window content: ContentView (sidebar + detail), Settings, AISettings, Welcome, History, Feedback, and supporting components. All under `FluidVoice-1.5.5/Sources/Fluid/` (root and UI/).

| Component | File | Purpose |
|-----------|------|--------|
| **ContentView** | ContentView.swift | Root: NavigationSplitView (sidebar + detail), hotkey/ASR wiring, sidebar selection, delay init, shortcut recording. |
| **SidebarItem** | ContentView.swift | Enum: welcome, voiceEngine, aiEnhancements, preferences, meetingTools, customDictionary, stats, history, feedback, commandMode, rewriteMode. |
| **SettingsView** | UI/SettingsView.swift | Preferences: audio devices, hotkeys, dock, launch at startup, overlay, debug, update. |
| **AISettingsView** | UI/AISettingsView.swift | AI provider/model picker, API keys, dictation prompt, advanced; split into extensions (AIConfiguration, SpeechRecognition, AdvancedSettings). |
| **AISettings (sub)** | UI/AISettings/*.swift | AIEnhancementSettingsView, VoiceEngineSettingsView, ViewModels, AISettingsScreens. |
| **WelcomeView** | UI/WelcomeView.swift | Onboarding / first-run: permissions, try dictation, copy or type. |
| **TranscriptionHistoryView** | UI/TranscriptionHistoryView.swift | List of TranscriptionHistoryEntry; copy, type again; delete. |
| **RecordingView** | UI/RecordingView.swift | Recording state UI (e.g. waveform or recording indicator). |
| **FeedbackView** | UI/FeedbackView.swift | User feedback / contact. |
| **AnalyticsPrivacyView** | UI/AnalyticsPrivacyView.swift | Analytics consent and privacy text. |
| **StatsView** | UI/StatsView.swift | Usage stats (typing WPM, transcription counts, etc.). |
| **CustomDictionaryView** | UI/CustomDictionaryView.swift | Custom dictionary entries for ASR. |
| **MeetingTranscriptionView** | UI/MeetingTranscriptionView.swift | Meeting transcription flow UI. |
| **PromptTextView** | UI/PromptTextView.swift | Text view for prompts (e.g. dictation prompt editor). |
| **SearchableModelPicker** | UI/SearchableModelPicker.swift | Searchable model list for provider. |
| **SearchableProviderPicker** | UI/SearchableProviderPicker.swift | Searchable provider list. |
| **MenuBarIconGenerator** | UI/MenuBarIconGenerator.swift | Template image for menu bar. |
| **MouseTracker** | UI/MouseTracker.swift | Mouse position tracking for UI (e.g. hover). |
| **GlossyEffects** | UI/GlossyEffects.swift | Visual effects for overlay or cards. |
| **FluidIcon** | UI/FluidIcon.swift | App icon view. |

## Navigation

- Sidebar selection: `selectedSidebarItem: SidebarItem?`; detail view switches on it (Welcome, Voice Engine, AI Enhancements, Preferences, etc.).
- Menu bar: MenuBarManager.requestedNavigationDestination sets sidebar item; ContentView.handleMenuBarNavigation(...) sets selectedSidebarItem and clears destination.

## Dependencies

- UI depends on AppServices, SettingsStore, Theme, Models; uses services (ASR, CommandMode, RewriteMode, NotchOverlayManager) via environment or shared instances.
