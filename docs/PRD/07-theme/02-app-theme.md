# App Theme

## Purpose

Defines the single AppTheme value (dark mode with configurable accent) and exposes it via SwiftUI environment so all views use consistent colors and gradients without hardcoding.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Theme/AppTheme.swift`
- `FluidVoice-1.5.5/Sources/Fluid/Theme/ThemeEnvironment.swift`

## Depends on

- `SwiftUI`; `AppKit` in SetupComponents if needed

## Consumed by

- FluidApp (sets .appTheme); ContentView and all child views (read @Environment(\.theme)).
- ThemedCard, ThemedGroupBox, SetupComponents (use theme for backgrounds and borders).

## Contract

### AppTheme

- Type: struct or enum with static/instance methods.
- Factory: `AppTheme.dark(accent: Color)` — accent used for buttons, links, highlights.
- Properties: background colors, secondary background, text colors, border colors, gradient(s) for overlays or headers. Exact names in implementation (e.g. primaryBackground, accentGradient).
- App is dark-only; no light theme variant in current codebase.

### ThemeEnvironment

- Environment key: `\.theme` of type AppTheme (or equivalent).
- Modifier: `.appTheme(_ theme: AppTheme)` — sets the key on the view hierarchy.
- Views access via `@Environment(\.theme) private var theme`.

### Accent

- SettingsStore.accentColorOption (enum or Int) maps to Color; passed to AppTheme.dark(accent:) when settings change. ContentView/root observes settings and reapplies theme.

## Invariants

- All user-facing surfaces should use theme colors, not hardcoded Color.primary or fixed hex.
- Assets (e.g. AccentColor.colorset) may still be used for asset catalog; theme takes precedence for runtime accent.
