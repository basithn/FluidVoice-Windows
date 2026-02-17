# Bottom Overlay View

## Purpose

Provides a secondary overlay anchored to the bottom of the screen (e.g. for meeting transcription or a persistent strip). Shown/hidden by NotchOverlayManager or a dedicated flag; content is context-dependent (meeting transcript, alternate dictation UX, etc.).

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Views/BottomOverlayView.swift`

## Depends on

- AppKit, SwiftUI
- NotchOverlayManager (isBottomOverlayVisible); MeetingTranscriptionService if used for meeting flow
- Theme; optional state from services

## Contract

- View is presented as a borderless, topmost window or SwiftUI overlay when isBottomOverlayVisible is true.
- Content: meeting transcription view (MeetingTranscriptionView) or generic container; dismiss button or Escape to hide.
- Position/size may respect SettingsStore overlay position and size (overlayBottomOffset, overlaySize).

## Invariants

- Only one bottom overlay active; hide when switching mode or when user dismisses.
- Does not block main window interaction except in its frame; focus and keyboard handling as per implementation.
