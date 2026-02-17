# Typing Service

## Purpose

Inserts a string into the currently focused application as if the user had typed it. Uses CGEvent unicode delivery (preferred, per-PID when possible) and falls back to clipboard (copy then Cmd+V). Requires Accessibility permission.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Services/TypingService.swift`

## Depends on

- `AppKit`, `ApplicationServices`, `Foundation`
- `AXIsProcessTrusted`, `AXUIElementCreateSystemWide`, `kAXFocusedUIElementAttribute`, `AXUIElementGetPid`, `NSWorkspace.shared.frontmostApplication`, `NSRunningApplication.activate(options:)`
- `CGEvent(keyboardEventSource:virtualKey:keyDown:)`, `CGEvent.keyboardSetUnicodeString`, `CGEvent.post(tap:pid:)` / `CGEvent.post(tap:)`
- `NSPasteboard.general`

## Consumed by

- ContentView (dictation: type final text); RewriteModeService (type rewritten text); CommandModeService (if a tool result is “type text”); WelcomeView and history views (copy or type from history).

## Contract

### Main API

- `typeTextInstantly(_ text: String)` — type into current focus target.
- `typeTextInstantly(_ text: String, preferredTargetPID: pid_t?)` — prefer typing into the app with the given PID (e.g. app that was focused when recording started), then fall back to system focus or HID.

### Focus and activation

- `static func captureSystemFocusedPID() -> pid_t?` — returns PID of the AX-focused element; nil if not trusted or no focus.
- `static func activateApp(pid: pid_t) -> Bool` — activates the app for that PID (except self); returns false for own app.

### Insertion pipeline (internal)

1. If `preferredTargetPID` set: try `insertTextBulkInstant(text, targetPID: preferredTargetPID)` (CGEvent unicode to that PID).
2. Else get `focusedPID` from `captureSystemFocusedPID()`; if non-nil, try `insertTextBulkInstant(text, targetPID: focusedPID)`.
3. Else try HID tap (CGEvent unicode, no PID).
4. On failure: clipboard fallback (save current clipboard, set string, post Cmd+V, restore clipboard).
- Bulk insert: builds keyDown/keyUp events with `keyboardSetUnicodeString` for each character (or batch); posts to PID or default tap.
- 200 ms delay before insert (after `typeTextInstantly` async block starts) to allow target app to be ready.

### Concurrency

- `isCurrentlyTyping` flag prevents overlapping calls; second call while first is in progress is skipped (logged).

## Invariants

- Must run only when `AXIsProcessTrusted()` is true; otherwise method returns early without typing.
- Clipboard fallback alters clipboard; original is restored after Cmd+V.
- Do not call from a context where the focused app is FluidVoice’s own window if the intent is to type into another app; use `preferredTargetPID` from the app that had focus when user triggered the action.

## Edge cases

- Empty string: no-op.
- Special characters / emoji: CGEvent unicode path supports UTF-16; clipboard path supports full string.
- Some apps ignore CGEvent or have custom input; clipboard Cmd+V is fallback.
- Elevated apps: typing into an elevated app from a non-elevated process may fail; document for support.
