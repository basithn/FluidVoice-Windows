# Product Summary

## What FluidVoice Is

FluidVoice is a **macOS menu-bar voice-to-text dictation app** with optional **AI post-processing**. The user triggers recording with a **global hotkey** (press-and-hold or tap); speech is **transcribed** by an on-device ASR model, optionally **cleaned up** by an LLM, then **typed into the focused app** or copied to clipboard.

## Core User Flows

| Flow | Trigger | Outcome |
|------|--------|--------|
| **Dictation** | Global hotkey (hold or tap) | Record → transcribe → (optional AI) → type into focused app or copy |
| **Command mode** | Command-mode hotkey (optional) | Voice command → LLM + MCP tools → execute (e.g. terminal, app actions) |
| **Rewrite mode** | Rewrite hotkey or selection | Selected text + voice instruction → LLM rewrite → type back or replace |
| **Write mode** | Rewrite hotkey, no selection | Voice request → LLM generates text → type into app |

## Feature Boundaries

- **In scope:** Global hotkey, ASR (Parakeet/Apple Speech/Whisper), AI dictation cleanup, type-into-app, command/rewrite/write modes, overlay UI, history, settings, Keychain API keys, auto-update.
- **Out of scope (for this PRD):** Server-side components, mobile, Windows/Linux (see `WINDOWS_PORT_ANALYSIS.md` for porting).

## Requirements (from codebase)

- **macOS 14.0+**, **Apple Silicon** (M1+) or **Intel** (Whisper-only for ASR).
- **Microphone** permission.
- **Accessibility** permission (required for global hotkey and typing into other apps).
- **Keychain** access for storing provider API keys (user must "Always allow" when prompted).

## Source of Truth

- **UI strings / copy:** In SwiftUI views and `SettingsStore` static helpers (e.g. `defaultDictationPromptBodyText()`).
- **Defaults:** `SettingsStore.Keys` and property getters with `defaults.string(forKey: Keys.xyz)` and fallbacks.
- **Provider list and base URLs:** `ModelRepository` (built-in IDs, default models, default base URLs).

See [02-architecture.md](./02-architecture.md) for module map and [03-glossary.md](./03-glossary.md) for terms.
