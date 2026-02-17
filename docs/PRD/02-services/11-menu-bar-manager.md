# Menu Bar Manager

## Purpose

Manages the macOS menu bar extra (NSStatusItem): icon, menu items (Start/Stop dictation, Preferences, History, Command Mode, Rewrite, Feedback, Check for Updates, Quit), and navigation to ContentView sidebar destinations. Can be configured with ASRService for Start/Stop state and actions.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Services/MenuBarManager.swift`
- `FluidVoice-1.5.5/Sources/Fluid/UI/MenuBarIconGenerator.swift` (template image for light/dark)
- Icons: `FluidVoice-1.5.5/Sources/Fluid/Assets.xcassets/MenuBarIcon.imageset/`

## Depends on

- `AppKit`
- `NSStatusItem`, `NSStatusBar.system`, `NSMenu`, `NSMenuItem`
- `SettingsStore` (e.g. showInDock for Quit behavior if needed)
- Optional: `ASRService` (set via `configure(asrService:)`) for isRunning and start/stop

## Consumed by

- FluidApp (injects `menuBarManager` as EnvironmentObject); ContentView uses it for sidebar navigation and reads `requestedNavigationDestination` to drive `selectedSidebarItem`.
- Menu bar "Preferences" / "History" etc. set `requestedNavigationDestination`; ContentView's `onChange` calls `handleMenuBarNavigation`.

## Contract

### API

- `@MainActor`; `ObservableObject` (or similar) with `requestedNavigationDestination: SidebarItem?` (or equivalent) so ContentView can react.
- `func initializeMenuBar()` — creates status item and menu; call once after app/window ready (ContentView onAppear).
- `func configure(asrService: ASRService)` — stores ASR reference; menu items can show "Start" vs "Stop" and trigger start/stop.
- Menu items: map to actions (e.g. start/stop, open Preferences, History, Command Mode, Rewrite, Feedback, Check for Updates, Quit). "Preferences" typically sets destination to `.preferences` and optionally brings main window to front.

### Navigation

- When user picks a menu item that corresponds to a sidebar item, set `requestedNavigationDestination = .preferences` (or .history, .commandMode, etc.). ContentView clears it after handling so it doesn't re-trigger.

## Invariants

- Initialize menu bar only after window/UI is ready to avoid window server issues (per codebase comment).
- Icon should be template image for correct appearance in light/dark menu bar.

## Edge cases

- If ASR not yet configured, Start/Stop may be disabled or show placeholder until `configure(asrService:)` is called.
- Quit: use `NSApplication.shared.terminate(nil)` or equivalent; respect "show in Dock" for visibility.
