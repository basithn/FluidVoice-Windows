# Content View

## Purpose

Root view of the main window: NavigationSplitView with sidebar (list of SidebarItem) and detail (Welcome, Voice Engine, AI Enhancements, Preferences, Meeting, Custom Dictionary, Stats, History, Feedback, Command Mode, Rewrite Mode). Owns hotkey manager lifecycle, ASR/recording state, shortcut recording UI, and the 1.5s delayed service initialization. Coordinates with MenuBarManager for navigation.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/ContentView.swift`

## Depends on

- AppKit, AVFoundation, Combine, CoreAudio, CoreGraphics, Security, SwiftUI
- AppServices, MenuBarManager (EnvironmentObject); SettingsStore.shared; CommandModeService, RewriteModeService (StateObject); Theme; GlobalHotkeyManager (State)
- NSEvent monitors (local) for shortcut recording and Escape; AXIsProcessTrusted for permission UI
- NotchOverlayManager, ActiveAppMonitor, TypingService

## Contract

### State (key)

- selectedSidebarItem: SidebarItem?; columnVisibility: NavigationSplitViewVisibility
- hotkeyManager: GlobalHotkeyManager?; hotkeyManagerInitialized: Bool
- hotkeyShortcut, commandModeHotkeyShortcut, rewriteModeHotkeyShortcut (from Settings, with local copy for recording)
- isRecordingForRewrite, isRecordingForCommand; isRecordingShortcut, isRecordingCommandModeShortcut, isRecordingRewriteShortcut
- pendingModifierFlags, pendingModifierKeyCode, pendingModifierOnly (for modifier-only shortcut)
- recordingAppInfo (name, bundleId, windowTitle) for "type into" target
- inputDevices, outputDevices, selectedInputUID, selectedOutputUID; visualizerNoiseThreshold
- Provider/model state: availableModelsByProvider, selectedModelByProvider, availableModels, selectedModel, currentProvider, savedProviders, selectedProviderID, providerAPIKeys
- enableDebugLogs, pressAndHoldModeEnabled, enableStreamingPreview, copyToClipboard; launchAtStartup, showInDock, showRestartPrompt; accessibilityPollingTask
- UserDefaults keys: accessibilityRestartFlagKey, hasAutoRestartedForAccessibilityKey

### Lifecycle

- onAppear: set appear = true; check accessibility; handle menu bar navigation; initialize menu bar; after 1.5s call appServices.signalUIReady(), audioObserver.startObserving(), asr.initialize(), menuBarManager.configure(asrService:), refreshDevices(); then create GlobalHotkeyManager with callbacks and start().
- Hotkey callbacks: startRecording → asr.startRecording(), show notch; stopAndProcess → asr.stopAndProcess(), optional AI, then type or copy, add history; commandMode → show command flow; rewriteMode → show rewrite flow; cancel → asr.cancelRecording() or dismiss overlay.
- Recreate GlobalHotkeyManager when hotkeyShortcut, commandModeShortcut, rewriteModeShortcut, or enabled flags change.

### Shortcut recording

- NSEvent.addLocalMonitorForEvents for keyDown and flagsChanged. When recording: capture keyCode and modifiers into HotkeyShortcut; on Escape cancel recording. When not recording: if event matches global hotkey, pass through (return nil so GlobalHotkeyManager handles) or trigger action; if Escape and overlay open, dismiss. Modifier-only: track modifier press; on release without other key, set shortcut to modifier-only.
- Persist to SettingsStore and update local state.

### Detail view

- Switch on selectedSidebarItem: WelcomeView, AISettingsView (voice engine), AISettingsView (AI enhancements), SettingsView, MeetingTranscriptionView, CustomDictionaryView, StatsView, TranscriptionHistoryView, FeedbackView, CommandModeView, RewriteModeView. Exact mapping in implementation.

### Invariants

- Do not access asr or audioObserver before signalUIReady() and the 1.5s delay. Single GlobalHotkeyManager instance at a time; stop() before recreating.
- Accessibility: if not trusted, show prompt or restart banner; polling task may recheck and update UI.

## Edge cases

- Overlay has focus: use preferredTargetPID (recordingAppInfo) when typing so text goes to the app that had focus when user started recording.
- Conflicting shortcuts: dictation vs command vs rewrite are distinct; user can set same combo for two (behavior per GlobalHotkeyManager order).
