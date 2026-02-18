# Product Requirements Document (PRD)

## 1. Product Overview

**FluidVoice for Windows** is a desktop application that enables hands-free voice-to-text dictation with optional AI enhancement. Users press-and-hold a global hotkey to record speech, which is transcribed locally (or via a cloud service) and typed directly into whatever application has focus.

### 1.1 Goals

- Replicate the core FluidVoice macOS experience on Windows 10/11.
- Offer fast, on-device speech recognition via Whisper as the default engine.
- Provide optional AI post-processing (grammar correction, rewriting, summarization) through pluggable LLM providers.
- Deliver a polished, non-intrusive UX — system tray icon, overlay feedback, minimal configuration.

### 1.2 Non-Goals (v1)

- Mobile support (Android / iOS).
- macOS or Linux builds (focus on Windows first).
- Real-time streaming transcription (v1 uses "record → transcribe" model).
- Acting as a general-purpose voice assistant.

---

## 2. Target Users

| Persona | Description |
|---------|-------------|
| **Power Typist** | Professionals who type extensively (writers, developers, data-entry) and want voice as a complement or alternative. |
| **Accessibility User** | Users with repetitive strain injuries or motor disabilities who need voice input across all apps. |
| **Knowledge Worker** | People drafting emails, documents, or messages who want faster throughput with AI polish. |

---

## 3. Core Features

### 3.1 Voice Dictation (MVP — must ship in v1)
 
| ID | Feature | Description | Priority | Status |
|----|---------|-------------|----------|--------|
| F-01 | **Global Push-to-Talk** | Configurable hotkey (e.g. `CapsLock`, `Right Ctrl`). Hold to record, release to transcribe. | P0 | ✅ Implemented |
| F-02 | **On-device ASR** | Whisper-based transcription running locally. Model size selectable (tiny → large). | P0 | ✅ Implemented |
| F-03 | **Type into Focused App** | Transcribed text is injected as synthetic keystrokes into the currently focused window/control. | P0 | ✅ Implemented |
| F-04 | **System Tray Presence** | App lives in the system tray with status icon, context menu (Settings, Quit, etc.). | P0 | ✅ Implemented |
| F-05 | **Overlay Feedback** | Lightweight overlay showing recording state, transcription progress, and result preview. | P0 | ⚠️ Audio-only |
| F-06 | **Audio Input Selection** | Choose microphone / input device from settings. | P1 | ⚠️ Config Only |

### 3.2 AI Enhancement

| ID | Feature | Description | Priority |
|----|---------|-------------|----------|
| F-07 | **AI Post-Processing** | After transcription, optionally send text to an LLM for grammar fix, rephrasing, or summarization. | P1 |
| F-08 | **Provider Configuration** | Support multiple AI backends: OpenAI, Groq, OpenRouter, Ollama (local), custom endpoint. | P1 |
| F-09 | **Prompt Customization** | Allow users to edit the system prompt / instruction used for AI post-processing. | P2 |

### 3.3 Text Manipulation Modes

| ID | Feature | Description | Priority |
|----|---------|-------------|----------|
| F-10 | **Rewrite Mode** | Select text in any app → activate → speak instruction → AI rewrites the selected text. | P2 |
| F-11 | **Write Mode** | No text selected → activate → speak → AI generates text from voice instruction. | P2 |
| F-12 | **Command Mode** | Voice commands that trigger system actions (e.g. "open browser", "next tab"). | P3 |

### 3.4 Settings & Data

| ID | Feature | Description | Priority |
|----|---------|-------------|----------|
| F-13 | **Settings Panel** | GUI for hotkey, model size, AI provider, theme, audio device. | P1 | ⬜ Future |
| F-14 | **History** | Searchable log of past transcriptions and AI outputs. | P2 | ⬜ Future |
| F-15 | **Secure API Key Storage** | Store provider API keys using Windows Credential Manager / DPAPI. | P1 | ⚠️ Config file only |
| F-16 | **Auto-Updater** | Check GitHub releases and offer in-app updates. | P2 | ⬜ Phase 5 |

### 3.5 Observability & Monitoring (Phase 4 — New)

| ID | Feature | Description | Priority | Status |
|----|---------|-------------|----------|--------|
| F-17 | **Structured Logging** | JSON log files in `%LOCALAPPDATA%`, timing instrumentation, error context | P0 | ⬜ Phase 4 |
| F-18 | **Crash Reporting** | Remote panic/error capture via Sentry | P1 | ⬜ Phase 4 |
| F-19 | **Heartbeat & Diagnostics** | Health ping, diagnostic zip export, audio quality metrics | P1 | ⬜ Phase 4 |
| F-20 | **Fleet Dashboard** | Backend to track active installs, versions, error rates | P2 | ⬜ Phase 5 |

---

## 4. User Stories

### Dictation

> **As a** knowledge worker, **I want to** hold a key and speak, **so that** my words appear as text in my current application without switching windows.

> **As a** user with RSI, **I want** speech-to-text to work in every app (browser, editor, terminal, chat), **so that** I can reduce keyboard usage across my workflow.

### AI Enhancement

> **As a** writer, **I want** my dictated text to be auto-corrected for grammar and punctuation, **so that** I can dictate without worrying about filler words or run-on sentences.

### Rewrite Mode

> **As a** professional, **I want to** select a paragraph I've written, speak a rewrite instruction, and have AI transform the text in-place, **so that** I can iterate on drafts hands-free.

---

## 5. Supported Platforms

| OS | Version | Status |
|----|---------|--------|
| Windows 11 | 22H2+ | Primary target |
| Windows 10 | 21H2+ | Supported |

---

## 6. Success Metrics (v1)

| Metric | Target | Status |
|--------|--------|--------|
| Dictation-to-type latency (Whisper base, modern hardware) | < 3 seconds for 10s audio clip | ✅ Met |
| App compatibility (type-into) | Works in Notepad, VS Code, Chrome, Slack | ✅ Met |
| Crash-free sessions | > 99 % | ✅ Met (limited testing) |
| Setup-to-first-dictation time | < 5 minutes | ✅ Met |

## 📈 Success Metrics (Observability — Phase 4+)

| Metric | Target |
|--------|--------|
| Any customer issue diagnosable from log files | ✅ |
| 100% of panics reported to Sentry | Within 24 hours |
| Know which version each customer runs | Via heartbeat |
| Mean-time-to-resolution for common issues | < 1 hour |
| Zero transcripts or PII transmitted | Privacy-first |
