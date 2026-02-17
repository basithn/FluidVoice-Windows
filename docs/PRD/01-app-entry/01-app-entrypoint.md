# App Entrypoint

## Purpose

Defines the SwiftUI app struct and root scene; wires `AppDelegate`, `AppServices`, and `MenuBarManager` into the view hierarchy.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/fluidApp.swift`

## Contract

- **Type:** `@main struct FluidApp: App`
- **Dependencies:** `AppKit`, `ApplicationServices`, `SwiftUI`
- **State:** `@StateObject private var menuBarManager = MenuBarManager()`, `@StateObject private var appServices: AppServices` (wrapped value: `AppServices.shared`), `@ObservedObject private var settings = SettingsStore.shared`, `@NSApplicationDelegateAdaptor(AppDelegate.self) var appDelegate`
- **Scene:** Single `WindowGroup(id: "main")` with `ContentView()`, `.environmentObject(menuBarManager)`, `.environmentObject(appServices)`, `.appTheme(AppTheme.dark(accent: settings.accentColor))`, `.preferredColorScheme(.dark)`, `.defaultSize(width: 1000, height: 700)`, `.windowResizability(.contentSize)`

## Invariants

- `AppServices.shared` is used so the same singleton is shared with ContentView and any child that reads `@EnvironmentObject private var appServices: AppServices`.
- No heavy work (ASR, CoreAudio) is done in `FluidApp.init()` or body; initialization is deferred to AppDelegate and ContentView’s delayed block.

## Consumed by

- System (SwiftUI lifecycle); ContentView receives `appServices` and `menuBarManager` via environment.
