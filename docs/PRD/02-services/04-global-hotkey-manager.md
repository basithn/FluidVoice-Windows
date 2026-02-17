# Global Hotkey Manager

## Purpose

Registers a system-wide (global) keyboard shortcut via CGEvent tap and invokes callbacks for: (1) dictation start/stop, (2) command mode, (3) rewrite mode. Supports press-and-hold (record while held) or tap (toggle) and modifier-only shortcuts. Requires Accessibility permission.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Services/GlobalHotkeyManager.swift`
- Shortcut model: `FluidVoice-1.5.5/Sources/Fluid/Models/HotkeyShortcut.swift`

## Depends on

- `AppKit`, `Foundation`
- `CGEvent`, `CGEventTap`, `NSEvent.ModifierFlags`
- `AXIsProcessTrusted()` — must be true to create tap
- `ASRService` (injected), `SettingsStore` (shortcuts, pressAndHoldMode)
- `HotkeyShortcut` (keyCode + modifierFlags)

## Consumed by

- ContentView creates and holds `GlobalHotkeyManager` in `@State`; passes callbacks for startRecording, stopAndProcess, commandMode, rewriteMode, cancel. Recreates when shortcuts or press-and-hold setting change.

## Contract

### Initialization

- `init(asrService: ASRService, shortcut: HotkeyShortcut, commandModeShortcut: HotkeyShortcut, rewriteModeShortcut: HotkeyShortcut, commandModeShortcutEnabled: Bool, rewriteModeShortcutEnabled: Bool, startRecordingCallback: (() async -> Void)?, stopAndProcessCallback: (() async -> Void)?, commandModeCallback: (() async -> Void)?, rewriteModeCallback: (() async -> Void)?)`
- Callbacks are optional; typically all provided from ContentView.

### Start/stop tap

- `start()` — creates CGEvent tap for keyDown/keyUp/flagsChanged; enables tap; runs on dedicated CFRunLoop. If `!AXIsProcessTrusted()`, logs error and does not create tap.
- `stop()` — disables and removes tap; run loop stops.

### Callback semantics

- **Dictation:** On shortcut keyDown (and modifiers match): `startRecordingCallback` (async). On keyUp (or modifier release in hold mode): `stopAndProcessCallback` (async). If press-and-hold: recording runs for the duration of the hold.
- **Command mode:** When command-mode shortcut matches: `commandModeCallback` (async); only if `commandModeShortcutEnabled`.
- **Rewrite mode:** When rewrite shortcut matches: `rewriteModeCallback` (async); only if `rewriteModeShortcutEnabled`.
- **Cancel:** Optional `cancelCallback: (() -> Bool)?`; if returns true, other handlers are not run (e.g. Escape or overlapping shortcut).

### HotkeyShortcut

- `keyCode: UInt16` (virtual key), `modifierFlags: NSEvent.ModifierFlags` (command, option, control, shift, function).
- Codable; stored in UserDefaults via SettingsStore keys: `HotkeyShortcutKey`, `CommandModeHotkeyShortcut`, `RewriteModeHotkeyShortcut`.
- `displayString` for UI (e.g. "⌘ ⇧ Space").

## Invariants

- Tap must be created on main thread; callbacks are invoked from the tap’s run loop and may need to dispatch to MainActor for UI/ASR.
- If tap is disabled (e.g. timeout or system), health-check logic may retry `start()`; see implementation for retry count and delay.
- Only one GlobalHotkeyManager should be active; ContentView holds a single instance and recreates it when shortcuts change.

## Edge cases

- Modifier-only shortcut: keyDown of modifier triggers “recording shortcut” state until keyUp or another key; handled via `modifierOnlyKeyDown` and `otherKeyPressedDuringModifier`.
- Conflicting shortcuts: dictation vs command vs rewrite are distinct; if two share the same combo, behavior is defined by order of checks in `handleKeyEvent`.
- Accessibility revoked: tap stops working; user must re-grant and possibly restart app.
