# Development Roadmap

## Overview

Development is organized into **four phases**, progressing from the foundational dictation loop to full feature parity with the macOS version. Each phase produces a usable milestone.

---

## Phase 1 — Foundation & Core Dictation (Weeks 1–6)

**Goal**: A working app that can record speech, transcribe with Whisper, and type text into the focused window.

| # | Task | Est. | Status |
|---|------|------|--------|
| 1.1 | Project scaffolding (Tauri 2 + React + Rust) | 2 days | ☐ |
| 1.2 | Global hotkey hook (`SetWindowsHookEx`) — press/release events | 3 days | ☐ |
| 1.3 | Audio capture via WASAPI (`cpal` crate), 16 kHz mono PCM | 3 days | ☐ |
| 1.4 | Whisper integration (`whisper-rs`), model loading, inference | 5 days | ☐ |
| 1.5 | Typing service — `SendInput()` Unicode keystrokes | 3 days | ☐ |
| 1.6 | Session controller — wire hotkey → capture → ASR → type | 3 days | ☐ |
| 1.7 | System tray icon + basic context menu (Quit, Settings stub) | 2 days | ☐ |
| 1.8 | Minimal overlay — recording/processing state indicator | 3 days | ☐ |
| 1.9 | Settings persistence (JSON in `%APPDATA%`) | 1 day | ☐ |
| 1.10 | Model download on first run | 2 days | ☐ |
| 1.11 | Phase 1 integration testing | 3 days | ☐ |

**Milestone**: `v0.1.0` — "Hold key → speak → text appears in Notepad/VS Code/Chrome"

---

## Phase 2 — AI Enhancement & Settings UI (Weeks 7–10)

**Goal**: Add AI post-processing, a proper settings panel, and secure API key storage.

| # | Task | Est. | Status |
|---|------|------|--------|
| 2.1 | AI Provider service — REST client for OpenAI, Groq, Ollama | 3 days | ☐ |
| 2.2 | Provider configuration UI (endpoint, API key, model, prompt) | 3 days | ☐ |
| 2.3 | Secure API key storage (Windows Credential Manager) | 2 days | ☐ |
| 2.4 | Settings panel UI — hotkey config, model selection, audio device | 4 days | ☐ |
| 2.5 | AI toggle — global on/off, per-session override | 1 day | ☐ |
| 2.6 | Overlay enhancements — show AI processing state, result preview | 2 days | ☐ |
| 2.7 | Audio device selection (enumerate + switch) | 2 days | ☐ |
| 2.8 | Phase 2 testing | 3 days | ☐ |

**Milestone**: `v0.2.0` — "Dictation with AI grammar correction, configurable settings"

---

## Phase 3 — Advanced Modes & History (Weeks 11–15)

**Goal**: Implement Rewrite, Write, and Command modes; add transcription history.

| # | Task | Est. | Status |
|---|------|------|--------|
| 3.1 | Get selected text (clipboard copy + restore technique) | 3 days | ☐ |
| 3.2 | Rewrite mode — select text → speak instruction → AI replaces | 4 days | ☐ |
| 3.3 | Write mode — no selection → speak → AI generates text | 3 days | ☐ |
| 3.4 | Command mode — voice commands → system actions | 5 days | ☐ |
| 3.5 | History DB (SQLite) — store transcriptions and AI outputs | 3 days | ☐ |
| 3.6 | History UI — searchable list with copy/re-type actions | 3 days | ☐ |
| 3.7 | Multiple hotkey profiles (dictation vs rewrite vs command) | 2 days | ☐ |
| 3.8 | Phase 3 testing | 4 days | ☐ |

**Milestone**: `v0.3.0` — "Full dictation modes + history"

---

## Phase 4 — Polish, Distribution & Parity (Weeks 16–20+)

**Goal**: Production-quality release — auto-updates, installer, edge-case hardening.

| # | Task | Est. | Status |
|---|------|------|--------|
| 4.1 | Auto-updater (Tauri updater plugin + GitHub Releases) | 3 days | ☐ |
| 4.2 | Windows installer (MSI or NSIS via Tauri bundler) | 2 days | ☐ |
| 4.3 | Typing edge-case hardening (elevated apps, UWP, Terminal, etc.) | 5 days | ☐ |
| 4.4 | Performance profiling — Whisper latency, memory usage | 3 days | ☐ |
| 4.5 | GPU acceleration for Whisper (CUDA / Vulkan via whisper.cpp) | 3 days | ☐ |
| 4.6 | Analytics integration (PostHog or equivalent) | 2 days | ☐ |
| 4.7 | Accessibility review & keyboard navigation | 2 days | ☐ |
| 4.8 | Documentation & onboarding flow | 2 days | ☐ |
| 4.9 | Beta testing & bug fixes | 5 days | ☐ |

**Milestone**: `v1.0.0` — "Production-ready Windows release"

---

## Timeline Summary

```
         Wk1    Wk4    Wk7    Wk10   Wk13   Wk16   Wk20
          │      │      │      │      │      │      │
Phase 1   ████████████████                                    v0.1.0
Phase 2                  ████████████                         v0.2.0
Phase 3                               ██████████████          v0.3.0
Phase 4                                              ████████ v1.0.0
```

---

## Key Dependencies

| Dependency | Phase | Risk |
|------------|-------|------|
| `whisper-rs` / `whisper.cpp` stable on Windows | Phase 1 | Medium — well-tested but GPU setup can be tricky |
| Tauri v2 tray + updater plugins | Phase 1 & 4 | Low — stable plugins |
| Windows Credential Manager access from Rust | Phase 2 | Low — `windows` crate supports `CredRead/CredWrite` |
| UI Automation for selected text | Phase 3 | Medium — app-specific behavior varies |
