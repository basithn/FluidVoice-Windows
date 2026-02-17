# Text Selection Service

## Purpose

Returns the currently selected text in the focused application using macOS Accessibility APIs. Used by Rewrite mode to get “selected text” for the LLM.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Services/TextSelectionService.swift`

## Depends on

- `AppKit`, `ApplicationServices`, `Foundation`
- `AXIsProcessTrusted`, `AXUIElementCreateSystemWide`, `AXUIElementCreateApplication(pid)`
- `kAXFocusedUIElementAttribute`, `kAXSelectedTextAttribute`
- `NSWorkspace.shared.frontmostApplication`

## Consumed by

- `RewriteModeService.captureSelectedText()` — gets selected text and sets `originalText` for rewrite.
- Any UI that needs to show or use “current selection” (e.g. rewrite flow).

## Contract

### API

- `static let shared = TextSelectionService()`
- `func getSelectedText() -> String?` — returns selected text or nil.

### Algorithm

1. If not `AXIsProcessTrusted()`, return nil.
2. Get system-wide focused element (`kAXFocusedUIElementAttribute` on system-wide element); get `kAXSelectedTextAttribute` from it; if non-nil string, return it.
3. Fallback: get frontmost application from `NSWorkspace.shared.frontmostApplication`; get app’s AX element; get focused element inside app; get selected text from that element; return if non-nil.
4. Otherwise return nil.

## Invariants

- No side effects (read-only). Returns nil if permission missing or app/element does not support selected text.
- Some apps expose selection via different AX attributes or not at all; fallback is best-effort.

## Edge cases

- Rich text: returns plain string representation when available.
- Multiple selections: AX typically exposes a single selected range; behavior is app-dependent.
- Password fields: may return empty or nil for security.
