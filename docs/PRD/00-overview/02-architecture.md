# Architecture

## High-Level Layers

```
┌─────────────────────────────────────────────────────────────────┐
│  App entry (fluidApp.swift, AppDelegate)                         │
├─────────────────────────────────────────────────────────────────┤
│  UI (ContentView, Settings, AISettings, Welcome, History, etc.)  │
│  Views (Notch, BottomOverlay, CommandModeView, RewriteModeView)  │
├─────────────────────────────────────────────────────────────────┤
│  Services (ASR, GlobalHotkey, Typing, NotchOverlay, Command,     │
│            Rewrite, MenuBar, SimpleUpdater, …)                   │
├─────────────────────────────────────────────────────────────────┤
│  Networking (AIProvider, LLMClient, ModelRepository,              │
│              ModelDownloader, FunctionCallingProvider)            │
├─────────────────────────────────────────────────────────────────┤
│  Persistence (SettingsStore, KeychainService,                    │
│               TranscriptionHistoryStore, ChatHistoryStore)        │
├─────────────────────────────────────────────────────────────────┤
│  Analytics (AnalyticsService, AnalyticsEvent, Identity, Config)   │
├─────────────────────────────────────────────────────────────────┤
│  Models (HotkeyShortcut, etc.)  │  Theme (AppTheme, components)   │
└─────────────────────────────────────────────────────────────────┘
```

## Dependency Rules

- **UI** depends on Services, Persistence, Theme, Models. UI must not call Networking directly for business logic; use Services (e.g. `ASRService`, `CommandModeService`) or `LLMClient`/`AIProvider` via services.
- **Services** may use Persistence (`SettingsStore`, Keychain, history stores), Networking (`LLMClient`, `ModelRepository`, providers), and other services. No service should depend on UI types (views).
- **Networking** is independent of UI; it may use Persistence for API keys (via `SettingsStore`/Keychain).
- **Persistence** has no dependency on Services or UI (plain UserDefaults, Keychain, file/codable).

## Singletons and Lifecycle

| Type | Singleton | Created |
|------|-----------|--------|
| `AppServices` | `AppServices.shared` | At launch; holds lazy `ASRService`, `AudioHardwareObserver` |
| `SettingsStore` | `SettingsStore.shared` | At first use |
| `KeychainService` | `KeychainService.shared` | At first use |
| `ASRService` | Via `AppServices.shared.asr` | After `signalUIReady()` + 1.5s delay (lazy) |
| `GlobalHotkeyManager` | Created in ContentView, held in `@State` | After ASR init, when shortcut is set |
| `CommandModeService` | `@StateObject` in ContentView | With ContentView |
| `RewriteModeService` | `@StateObject` in ContentView | With ContentView |
| `NotchOverlayManager` | `NotchOverlayManager.shared` | On first use |
| `ActiveAppMonitor` | `ActiveAppMonitor.shared` | On first use |
| `LLMClient` | `LLMClient.shared` | On first use |
| `ModelRepository` | `ModelRepository.shared` | On first use |
| `ChatHistoryStore` | `ChatHistoryStore.shared` | On first use |
| `TranscriptionHistoryStore` | `TranscriptionHistoryStore.shared` | On first use |

## Startup Sequence (Critical)

1. `FluidApp` body runs; `AppDelegate.applicationDidFinishLaunching` runs (accessibility prompt, settings init, analytics bootstrap, update timer).
2. `ContentView` appears; **no** heavy audio/ASR work yet.
3. After **1.5s** delay: `appServices.signalUIReady()` → `audioObserver.startObserving()` → `asr.initialize()` → menu bar configured with ASR.
4. Lazy services (ASR, audio observer) are created on first access **after** `isUIReady == true`.

This order avoids a known race (CoreAudio HAL vs SwiftUI AttributeGraph) that can cause EXC_BAD_ACCESS at launch. Do not initialize ASR or start CoreAudio observers before the 1.5s + `signalUIReady()`.

## Threading

- **Main thread / @MainActor:** All UI, `ASRService` public API, `GlobalHotkeyManager`, `CommandModeService`, `RewriteModeService`, `SettingsStore` observable updates, overlay show/hide.
- **Background:** Audio capture and processing (internal to ASR/audio pipeline), model download (`ModelDownloader`), some file I/O. Results are dispatched back to main for UI updates.
- **TranscriptionExecutor (actor):** Serializes CoreML/transcription work inside ASR to avoid concurrent model access.

## Source Files (by layer)

- **App entry:** `fluidApp.swift`, `AppDelegate.swift`
- **UI:** `ContentView.swift`, `UI/*.swift`, `UI/AISettings/*.swift`, `Theme/*.swift`
- **Views:** `Views/NotchContentViews.swift`, `Views/BottomOverlayView.swift`, `Views/CommandModeView.swift`, `Views/RewriteModeView.swift`
- **Services:** `Services/*.swift` (see [02-services/01-services-index.md](../02-services/01-services-index.md))
- **Networking:** `Networking/*.swift`
- **Persistence:** `Persistence/*.swift`
- **Analytics:** `Analytics/*.swift`
- **Models:** `Models/HotkeyShortcut.swift`

All paths relative to `FluidVoice-1.5.5/Sources/Fluid/`.
