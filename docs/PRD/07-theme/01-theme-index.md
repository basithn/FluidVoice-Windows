# Theme Index

Theme covers app-wide visual styling: colors, typography, and reusable SwiftUI components. All under `FluidVoice-1.5.5/Sources/Fluid/Theme/`.

| Component | File | Purpose |
|-----------|------|--------|
| **AppTheme** | AppTheme.swift | Central theme type: dark(accent:), colors, gradients. |
| **ThemeEnvironment** | ThemeEnvironment.swift | SwiftUI environment key for theme injection. |
| **NativeButtonStyles** | NativeButtonStyles.swift | Button styles (primary, secondary, destructive, etc.). |
| **ThemedCard** | Theme/Components/ThemedCard.swift | Card container with theme background/border. |
| **ThemedGroupBox** | Theme/Components/ThemedGroupBox.swift | GroupBox-style container. |
| **SetupComponents** | Theme/Components/SetupComponents.swift | Shared setup/onboarding UI pieces. |

## Usage

- Root: FluidApp applies `.appTheme(AppTheme.dark(accent: settings.accentColor))` and `.preferredColorScheme(.dark)`.
- Views read `@Environment(\.theme) private var theme` and use theme colors/gradients.
- Accent comes from SettingsStore.accentColorOption; theme is rebuilt when accent changes.
