# Notch Content Views

## Purpose

SwiftUI views that fill the DynamicNotch overlay: compact (leading/trailing) and expanded for dictation/rewrite; separate expanded view for command-mode output (chat + tool results). Read from NotchContentState and optional audio publisher; trigger callbacks (dismiss, follow-up, new chat, etc.) via NotchOverlayManager.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Views/NotchContentViews.swift`
- State: NotchContentState (shared) — likely in same file or Services

## Depends on

- AppKit, SwiftUI, DynamicNotchKit
- NotchContentState.shared (targetAppIcon, transcription text, command messages, recent chats)
- NotchOverlayManager callbacks (onNotchClicked, onCommandOutputDismiss, onCommandFollowUp, onNewChat, onSwitchChat, onClearChat)
- Optional: audio level publisher for visualization in compact/expanded

## Contract

### NotchExpandedView

- Shown when notch is expanded in dictation or rewrite mode. Displays: live or final transcription, target app icon, optional audio level. Buttons or gestures: dismiss (Escape or click), possibly "Type" or "Copy". Uses theme and overlay-safe layout.

### NotchCompactLeadingView / NotchCompactTrailingView

- Small pill or icon when notch is compact; may show mic icon or level. Click expands (or triggers onNotchClicked).

### NotchCommandOutputExpandedView

- Command mode: list of messages (user, assistant, tool); streaming text; thinking tokens if enabled; pending command with Confirm/Cancel; New Chat, Switch Chat, Clear. Calls onCommandOutputDismiss (Escape/close), onCommandFollowUp (text), onNewChat, onSwitchChat(id), onClearChat.

### NotchContentState

- Observable or static shared state: targetAppIcon (NSImage/Image), current transcription string, command messages array, recentChats, selectedChatID. Written by ActiveAppMonitor, ASRService path, CommandModeService, NotchOverlayManager. Read by these views.

## Invariants

- Views must not block main thread; streaming updates via @Published or Binding from services.
- Escape and click-dismiss must call manager callbacks so overlay can hide and cancel if needed.
