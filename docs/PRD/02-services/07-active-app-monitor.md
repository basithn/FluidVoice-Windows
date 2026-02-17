# Active App Monitor

## Purpose

Tracks the currently frontmost (active) application and publishes its reference, icon, bundle ID, and display name. Used by overlays and command/rewrite flows to show “target app” or to pass preferred PID to TypingService.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Services/ActiveAppMonitor.swift`

## Depends on

- `AppKit`, `Combine`
- `NSWorkspace.shared.frontmostApplication`, `NSWorkspace.didActivateApplicationNotification`
- `NotchContentState.shared` (writes `targetAppIcon` for overlay)

## Consumed by

- NotchOverlayManager / notch views (show target app icon); ContentView and flows that need “current app” for display or for `TypingService.typeTextInstantly(_, preferredTargetPID:)`.
- `NotchContentState.shared.targetAppIcon` is set here when active app changes.

## Contract

### API

- `static let shared = ActiveAppMonitor()`
- `@Published private(set) var activeApp: NSRunningApplication?`
- `@Published private(set) var activeAppIcon: NSImage?`
- `var activeAppBundleID: String?` — `activeApp?.bundleIdentifier`
- `var activeAppName: String?` — `activeApp?.localizedName`
- `func startMonitoring()` — subscribes to `NSWorkspace.didActivateApplicationNotification`, updates state.
- `func stopMonitoring()` — unsubscribes, clears state.
- `func refreshActiveApp()` — one-shot update.

### Behavior

- On each activation notification (or refresh), sets `activeApp` and `activeAppIcon` from `NSWorkspace.shared.frontmostApplication`.
- Excludes self: if frontmost app’s bundle ID equals FluidVoice’s, does not update (keeps previous non-self app or nil).
- Only updates when `activeApp?.bundleIdentifier != frontApp.bundleIdentifier` to avoid redundant publishes.

## Invariants

- Must call `startMonitoring()` when overlay or feature needs live updates; call `stopMonitoring()` when not needed to avoid unnecessary work.
- Thread: notification handler dispatches to MainActor for state updates; class is ObservableObject on main.
