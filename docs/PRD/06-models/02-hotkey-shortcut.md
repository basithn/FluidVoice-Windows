# Hotkey Shortcut Model

## Purpose

Represents a single global hotkey: virtual key code plus modifier flags (Command, Option, Control, Shift, Function). Used for dictation, command mode, and rewrite mode shortcuts. Codable for UserDefaults persistence; display string for UI.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Models/HotkeyShortcut.swift`

## Depends on

- `AppKit`, `Foundation`
- NSEvent.ModifierFlags; UInt16 keyCode

## Consumed by

- SettingsStore (hotkeyShortcut, commandModeHotkeyShortcut, rewriteModeHotkeyShortcut); GlobalHotkeyManager (init and match logic); ContentView (shortcut recording UI); HotkeyShortcut.displayString in settings and recording views.

## Contract

### Type

- `struct HotkeyShortcut: Codable, Equatable`
- `var keyCode: UInt16` — virtual key (e.g. 49 = Space, 36 = Return).
- `var modifierFlags: NSEvent.ModifierFlags` — .command, .option, .control, .shift, .function.

### Coding

- CodingKeys: keyCode, modifierFlagsRawValue (UInt for NSEvent.ModifierFlags.rawValue).
- init(from decoder): decode keyCode and rawValue; set modifierFlags = ModifierFlags(rawValue: raw).
- encode(to encoder): encode keyCode and modifierFlags.rawValue.

### Display

- `var displayString: String` — e.g. "⌘ ⇧ Space". Order: function (🌐), command (⌘), option (⌥), control (⌃), shift (⇧), then key from keyCodeToString(keyCode). If no modifiers, return key only.
- `static func keyCodeToString(_ keyCode: UInt16) -> String?` — map common keyCodes to readable strings (Return, Tab, Space, Delete, Escape, Left/Right modifiers, letters, digits, symbols). Returns nil for unknown; then displayString uses Character(UnicodeScalar(keyCode)) or "?".

### Key code constants (examples)

- 36 Return, 48 Tab, 49 Space, 51 Delete, 53 Escape; 55/54 Left/Right Command; 56/60 Left/Right Shift; 59/62 Left/Right Control; 58/61 Left/Right Option; 63 fn; 123–126 arrows; 0–50 for letters and symbols (see implementation).

## Invariants

- Same HotkeyShortcut value must match in GlobalHotkeyManager (compare keyCode and modifierFlags intersection with [.function, .command, .option, .control, .shift]).
- Persisted keys: HotkeyShortcutKey, CommandModeHotkeyShortcut, RewriteModeHotkeyShortcut (SettingsStore).
