# Notch Overlay Manager

## Purpose

Controls the DynamicNotchKit-based floating “notch” overlay: show/hide for dictation (with optional audio level publisher), command output, and rewrite result. Manages escape key monitors and callbacks for dismiss, follow-up, and chat actions.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Services/NotchOverlayManager.swift`
- Views: `FluidVoice-1.5.5/Sources/Fluid/Views/NotchContentViews.swift` (NotchExpandedView, NotchCompactLeadingView, NotchCompactTrailingView, NotchCommandOutputExpandedView)

## Depends on

- `AppKit`, `Combine`, `SwiftUI`, `DynamicNotchKit`
- `NSEvent.addGlobalMonitorForEvents`, `NSEvent.addLocalMonitorForEvents`, `NSEvent.removeMonitor`
- `NotchContentState` (shared state for overlay content), `ActiveAppMonitor` (optional for icon)
- ASR audio level publisher (from ASRService or equivalent) for visualization

## Consumed by

- ContentView and flow code: call `showNotch(...)`, `hideNotch()`, `showCommandOutput(...)`, `hideCommandOutput()`; set callbacks (onCommandOutputDismiss, onCommandFollowUp, onNotchClicked, onNewChat, onSwitchChat, onClearChat).
- Notch content views read from `NotchContentState` and callbacks.

## Contract

### Modes

- `OverlayMode`: `.dictation`, `.rewrite`, `.write`, `.command`
- Main notch: dictation preview, rewrite/write result, or entry point to command.
- Command output notch: expanded view for command-mode conversation and tool output.

### Show/hide

- **Dictation/rewrite/write:** `showNotch(...)` with mode, optional audio publisher; `hideNotch()`.
- **Command output:** `showCommandOutput(...)`; `hideCommandOutput()`.
- State machine: idle → showing → visible → hiding → idle; generation counter used to avoid race conditions from rapid show/hide.

### Escape key

- Global and local monitors for keyDown; keyCode 53 (Escape). On Escape: if command output expanded, hide it and call `onCommandOutputDismiss`; else hide main notch and run cancel/dismiss as appropriate. Monitors are removed in deinit.

### Callbacks

- `onCommandOutputDismiss`, `onCommandFollowUp(String)`, `onNotchClicked`
- `onNewChat`, `onSwitchChat(String)`, `onClearChat`
- Set by ContentView / CommandModeService; invoked on main thread.

### Audio visualization

- `currentAudioPublisher: AnyPublisher<CGFloat, Never>?` — optional; when set, compact/expanded views can show level. Stored for re-show during processing.

## Invariants

- Only one main notch and one command output notch at a time; show/hide are not reentrant (state machine + generation).
- Escape handling must not block; callbacks may be async.

## Edge cases

- Notch clicked while recording: behavior defined by `onNotchClicked` (e.g. focus main window or no-op).
- Multiple rapid show requests: later show may overwrite mode/content; hide then show for clean state.
