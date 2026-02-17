# Views Index

Views covers overlay and mode-specific views: notch content, bottom overlay, command mode view, rewrite mode view. All under `FluidVoice-1.5.5/Sources/Fluid/Views/` and related UI.

| View | File | Purpose |
|------|------|--------|
| **NotchContentViews** | Views/NotchContentViews.swift | NotchExpandedView, NotchCompactLeadingView, NotchCompactTrailingView (dictation/rewrite); NotchCommandOutputExpandedView (command output). Content for DynamicNotch. |
| **BottomOverlayView** | Views/BottomOverlayView.swift | Bottom overlay container (e.g. meeting transcription or alternate UX). |
| **CommandModeView** | Views/CommandModeView.swift | Command mode UI: input, chat history, tool output, confirm/cancel; uses CommandModeService. |
| **RewriteModeView** | Views/RewriteModeView.swift | Rewrite/write mode UI: original text, prompt, streamed result, type/copy; uses RewriteModeService. |

## State sources

- NotchContentState (shared): target app icon, transcription text, command output messages, recent chats; cleared or updated by NotchOverlayManager and CommandModeService.
- NotchOverlayManager: show/hide, mode, audio level publisher; callbacks for dismiss, follow-up, new chat, switch chat, clear chat.
- CommandModeService: conversationHistory, isProcessing, pendingCommand, streamingText, currentStep.
- RewriteModeService: originalText, rewrittenText, isProcessing, conversationHistory, isWriteMode.

## Dependencies

- AppKit (for NotchContentViews if needed); SwiftUI; Theme; DynamicNotchKit (notch); services and stores as above.
