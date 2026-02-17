# Rewrite Mode Service

## Purpose

Handles “rewrite” and “write” flows: (1) User selects text and gives a voice instruction → LLM rewrites the selection; (2) User gives a voice instruction with no selection → LLM generates text (write mode). Result is streamed and can be typed into the app via TypingService or shown in overlay.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Services/RewriteModeService.swift`
- `FluidVoice-1.5.5/Sources/Fluid/Services/TextSelectionService.swift` (selection)
- `FluidVoice-1.5.5/Sources/Fluid/Services/TypingService.swift` (insert result)

## Depends on

- `AppKit`, `Combine`, `Foundation`
- `LLMClient` (or provider abstraction) for chat completion; `TextSelectionService.shared`; `TypingService()`; `SettingsStore` (provider, model, API key)
- Optional: NotchContentState for overlay display

## Consumed by

- ContentView; RewriteModeView; GlobalHotkeyManager (rewrite shortcut); NotchOverlayManager (rewrite result in overlay).

## Contract

### State (published)

- `originalText: String` — selected text or (in write mode) user’s prompt.
- `rewrittenText: String` — LLM output.
- `streamingThinkingText: String` — thinking tokens for UI.
- `isProcessing: Bool`
- `conversationHistory: [Message]` — for multi-turn rewrite.
- `isWriteMode: Bool` — true when no selection (write), false when rewriting selection.

### Methods

- `captureSelectedText() -> Bool` — gets selection via TextSelectionService; if non-empty sets `originalText`, clears `rewrittenText`, sets `isWriteMode = false`; returns true if selection was set.
- `startWithoutSelection()` — clears original/rewritten; sets `isWriteMode = true` (user will speak request).
- `setOriginalText(_ text: String)` — for write mode when original comes from voice.
- `processRewriteRequest(_ prompt: String) async` — if `originalText.isEmpty`, treats prompt as write request (appends to conversation, calls LLM to generate). Else builds rewrite prompt (“Here is the text… User’s instruction: …”), appends to conversation, streams LLM response into `rewrittenText` and optional thinking.
- `typeResult()` — calls `TypingService().typeTextInstantly(rewrittenText)` (optionally with preferred PID from focus when rewrite was triggered).

### Message model

- `Message(role: .user | .assistant, content: String)`; conversation history used for follow-up rewrites.

## Invariants

- Original text for rewrite must be non-empty when `processRewriteRequest` is called for rewrite (not write) mode; write mode uses prompt as the request.
- Typing uses same accessibility and PID rules as main TypingService doc.

## Edge cases

- Empty selection and user triggers rewrite: can show “Select text first” or auto-switch to write mode (implementation detail).
- Very long selection: may hit token limits; truncation or chunking is implementation-specific.
- Streamed result: UI may show partial `rewrittenText` until done; typeResult uses final text.
