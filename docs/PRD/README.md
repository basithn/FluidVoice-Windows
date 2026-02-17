# FluidVoice PRD (Product Requirements Document)

**Purpose:** Machine- and human-readable product and technical specification derived from the FluidVoice macOS codebase. Optimized for agentic development workflows: agents can resolve file locations, contracts, dependencies, and invariants from this PRD without guessing.

**Source:** `FluidVoice-1.5.5/Sources/Fluid/` (Swift, SwiftUI, AppKit).  
**Platform:** macOS 14+ (Sonoma), Apple Silicon preferred; Intel supported via Whisper models.

---

## PRD Structure

| Folder | Scope |
|--------|--------|
| [00-overview](./00-overview/) | Product summary, architecture, glossary, doc conventions |
| [01-app-entry](./01-app-entry/) | App entrypoint, AppDelegate, lifecycle |
| [02-services](./02-services/) | All business-logic services (ASR, hotkey, typing, overlay, command, rewrite, etc.) |
| [03-networking](./03-networking/) | AI/LLM providers, model repo, model downloader, function calling |
| [04-persistence](./04-persistence/) | Settings, Keychain, transcription history, chat history |
| [05-analytics](./05-analytics/) | Events, schema, AnalyticsService |
| [06-models](./06-models/) | Shared data models (e.g. HotkeyShortcut) |
| [07-theme](./07-theme/) | AppTheme, components, environment |
| [08-ui](./08-ui/) | Main UI: ContentView, Settings, AISettings, Welcome, History, etc. |
| [09-views](./09-views/) | Overlay views: Notch, BottomOverlay, CommandModeView, RewriteModeView |

---

## How to Use (Agents)

1. **Resolve source file:** Each module doc lists `Source files` with paths relative to repo root (e.g. `FluidVoice-1.5.5/Sources/Fluid/...`). Use these for edits and references.
2. **Dependencies:** `Depends on` and `Consumed by` define allowed call directions; respect layering when adding code.
3. **Contracts:** Types, method signatures, and keys are specified so implementations and tests can be generated or verified.
4. **Invariants & edge cases:** Documented where behavior is subtle (e.g. startup order, accessibility, threading).
5. **Cross-references:** Use `[Module: doc](path)` links to jump between PRD sections.

---

## Conventions

- **Source paths:** Given as `FluidVoice-1.5.5/Sources/Fluid/<path>` unless noted.
- **Swift types:** Referenced by exact name (e.g. `ASRService`, `SettingsStore.shared`).
- **Keys:** UserDefaults/Keychain keys are literal strings as in code (e.g. `HotkeyShortcutKey`).
- **Threading:** `@MainActor` is stated where applicable; background work is called out.

See [00-overview/04-doc-conventions.md](./00-overview/04-doc-conventions.md) for full conventions.
