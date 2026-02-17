# App Delegate

## Purpose

Handles app lifecycle: accessibility permission prompt, settings init, analytics bootstrap, automatic and manual update checks, and periodic update timer. Does not create ASR or audio services.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/AppDelegate.swift`

## Depends on

- `AppKit`, `AppUpdater`, `PromiseKit`, `SwiftUI`
- `SettingsStore.shared`, `AnalyticsService`, `AnalyticsIdentityStore`, `DebugLogger`
- `SimpleUpdater` (manual check), `AXIsProcessTrusted`, `AXIsProcessTrustedWithOptions`, `NSWorkspace`, `NSAlert`

## Contract

### applicationDidFinishLaunching

- Calls `requestAccessibilityPermissions()` (debounced; may open System Preferences → Privacy → Accessibility if still untrusted after 1.2s).
- Calls `SettingsStore.shared.initializeAppSettings()`.
- Calls `AnalyticsService.shared.bootstrap()`.
- If first open: `AnalyticsIdentityStore.shared.ensureFirstOpenRecorded()` then `AnalyticsService.shared.capture(.appFirstOpen)`.
- Always: `AnalyticsService.shared.capture(.appOpen, properties: ["accessibility_trusted": AXIsProcessTrusted()])`.
- Calls `checkForUpdatesAutomatically()` (respects `SettingsStore.shouldCheckForUpdates()`).
- Schedules `schedulePeriodicUpdateChecks()` (timer every 3600s).

### applicationWillTerminate

- Invalidates `updateCheckTimer`, sets to `nil`.

### requestAccessibilityPermissions()

- If `AXIsProcessTrusted()` already: return.
- If `AXPromptState.hasPromptedThisSession`: return.
- Cooldown: do not prompt again within 24h (UserDefaults key `AXLastPromptAt`).
- Calls `AXIsProcessTrustedWithOptions([kAXTrustedCheckOptionPrompt: true])`.
- After 1.2s, if still not trusted, opens `x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility`.

### checkForUpdatesManually()

- Uses `SimpleUpdater.shared.checkAndUpdate(owner: "altic-dev", repo: "Fluid-oss")`; on success shows “Update Found!” and updater handles relaunch; on cancel shows “No Updates”; on error shows “Update Check Failed” with message.

### Automatic update check

- `checkForUpdatesAutomatically()` runs on timer and on launch; only runs if `SettingsStore.shared.shouldCheckForUpdates()` is true. Calls `SimpleUpdater.shared.checkForUpdate(...)`; if update available and not snoozed, shows `showUpdateNotification(version:)` (Install Now / Later). “Later” snoozes for 24h for that version.

## Invariants

- No ASR or audio initialization here; that is in ContentView after delay + `AppServices.signalUIReady()`.
- Update repo is fixed: `altic-dev/Fluid-oss`.

## Consumed by

- System (NSApplicationDelegate); not referenced by other app code directly.
