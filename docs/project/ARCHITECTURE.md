# Architecture & Technology Decisions

## 1. High-Level Architecture

FluidVoice for Windows is a **desktop application** composed of four main layers:

```
┌──────────────────────────────────────────────────────┐
│                    Presentation                      │
│   System Tray  ·  Overlay Window  ·  Settings UI     │
├──────────────────────────────────────────────────────┤
│                  Application Core                    │
│   Hotkey Manager · Session Controller · History DB   │
├──────────────────────────────────────────────────────┤
│                  Service Layer                       │
│   Audio Capture · ASR Engine · AI Provider · Typing  │
├──────────────────────────────────────────────────────┤
│              Platform Integration                    │
│   Win32 Hooks · SendInput · UI Automation · WASAPI   │
│   Credential Manager · Shell_NotifyIcon              │
└──────────────────────────────────────────────────────┘
```

### Component Descriptions

| Layer | Component | Responsibility |
|-------|-----------|----------------|
| **Presentation** | System Tray | App lifecycle, status icon, quick-access menu |
| | Overlay Window | Visual recording/transcription feedback (topmost, transparent, borderless) |
| | Settings UI | Configuration panels for hotkey, model, AI providers, audio device |
| **Core** | Hotkey Manager | Registers global hotkey, emits press/release events |
| | Session Controller | Orchestrates record → transcribe → (optionally enhance) → type workflow |
| | History DB | Persists past transcriptions and AI outputs (SQLite) |
| **Services** | Audio Capture | Records mic input to PCM buffer (WASAPI) |
| | ASR Engine | Runs Whisper (or alternative) on audio buffer, returns text |
| | AI Provider | Sends text to configured LLM endpoint, returns enhanced text |
| | Typing Service | Injects text keystrokes into the focused window/control |
| **Platform** | Win32 / OS APIs | Low-level Windows integration (hooks, input, tray, credentials) |

---

## 2. Tech Stack Decision

### Recommended: **Tauri 2 + Rust (backend) + TypeScript/React (frontend)**

| Factor | Rationale |
|--------|-----------|
| **Binary size** | Tauri produces small binaries (~5–15 MB) vs Electron (~150 MB+). |
| **System access** | Rust has excellent Win32/COM bindings (`windows` crate) for hooks, SendInput, WASAPI, UI Automation. |
| **ASR integration** | `whisper-rs` (Rust bindings to whisper.cpp) provides native, in-process Whisper inference. |
| **UI flexibility** | Web-based frontend (React) allows rapid UI iteration for overlay, settings, and history. |
| **Cross-platform potential** | If a Linux port is ever desired, Tauri + Rust is well-positioned. |
| **Community & ecosystem** | Tauri v2 is stable, actively maintained, has plugin ecosystem (tray, updater, etc.). |

### Alternative stacks considered

| Stack | Pros | Cons |
|-------|------|------|
| C# + WinUI 3 | Best Windows-native integration | No cross-platform; C# ecosystem for Whisper is less mature |
| Electron + TypeScript | Large ecosystem, rapid UI dev | Heavy runtime, worse system integration for hooks/typing |
| Rust + egui / iced | Pure Rust, no web overhead | Less mature UI, harder to build polished overlay & settings |

---

## 3. Key Component Design

### 3.1 Global Hotkey (Hotkey Manager)

- Use `SetWindowsHookEx(WH_KEYBOARD_LL, ...)` for a low-level keyboard hook.
- Detect configured key press and release events.
- Emit `HotkeyPressed` / `HotkeyReleased` events to the Session Controller.
- **Consideration**: UIPI — the app should run at the same or higher integrity level as target apps; may need "Run as Administrator" for elevated targets.

### 3.2 Audio Capture

- Use **WASAPI** (via `cpal` or `wasapi` Rust crate) for low-latency microphone capture.
- Capture PCM audio at 16 kHz mono (Whisper's native sample rate) into a ring buffer.
- Start capture on `HotkeyPressed`, stop and flush on `HotkeyReleased`.

### 3.3 ASR Engine

- **Primary**: `whisper-rs` (Rust bindings to `whisper.cpp`).
  - Bundle model files (e.g. `ggml-base.en.bin`) or download on first run.
  - Run inference in a dedicated thread to avoid blocking UI.
- **Optional (future)**: Windows built-in Speech Recognition or Azure Cognitive Services for cloud-based option.

### 3.4 AI Provider Service

- REST client calling LLM APIs (OpenAI, Groq, OpenRouter, Ollama).
- Configurable provider URL, API key, model ID, and system prompt.
- Toggle on/off per session or globally.
- API keys stored via Windows Credential Manager (`CredRead` / `CredWrite`).

### 3.5 Typing Service ("Type into Any App")

- **Primary**: `SendInput()` with `KEYBDINPUT` for each character (supports Unicode via `KEYEVENTF_UNICODE`).
- **Fallback**: Clipboard paste (`SetClipboardData` + synthetic `Ctrl+V`) for apps that don't accept `SendInput`.
- **Get selected text** (for Rewrite mode): `Ctrl+C` → read clipboard → restore clipboard.
- **Determine focused app**: `GetForegroundWindow()` + `GetWindowThreadProcessId()`.

### 3.6 Overlay Window

- Transparent, topmost, click-through borderless window (Tauri `WebviewWindow` with `transparent: true`, `always_on_top: true`, `decorations: false`).
- Positioned at top-center of the primary monitor (emulating macOS "notch" style).
- Shows: recording indicator, live waveform/volume, transcription result preview.

### 3.7 System Tray

- Tauri's tray plugin (`tauri-plugin-shell` / tray API).
- Icon states: idle, recording, processing.
- Context menu: Settings, History, Check for Updates, Quit.

### 3.8 Data Storage

- **History**: SQLite via `rusqlite` or `sqlx`.
- **Settings**: JSON config file in `%APPDATA%\FluidVoice\`.
- **API keys**: Windows Credential Manager (not written to disk in plaintext).

### 3.9 Auto-Updater

- Tauri's built-in updater plugin (`tauri-plugin-updater`).
- Checks GitHub Releases for the latest version.
- Downloads and installs update, prompts user to restart.

---

## 4. Data Flow — Dictation Session

```
User holds hotkey
       │
       ▼
┌─────────────┐
│ Hotkey Mgr  │── HotkeyPressed ──▶ Session Controller
└─────────────┘                           │
                                          ▼
                                   Start Audio Capture
                                          │
User releases hotkey                      │
       │                                  │
       ▼                                  ▼
┌─────────────┐                    Stop Audio Capture
│ Hotkey Mgr  │── HotkeyReleased ──▶ Flush PCM buffer
└─────────────┘                           │
                                          ▼
                                   ┌─────────────┐
                                   │  ASR Engine  │
                                   │  (Whisper)   │
                                   └──────┬──────┘
                                          │ raw text
                                          ▼
                                   ┌─────────────┐
                               ┌──▶│ AI Provider  │──▶ enhanced text ─┐
                               │   └─────────────┘                    │
                      AI on?───┤                                      │
                               │                                      ▼
                               └─── raw text ────────────────▶ Typing Service
                                                                      │
                                                                      ▼
                                                              SendInput() to
                                                              focused window
```

---

## 5. Directory Structure (Proposed)

```
FluidVoice/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── hotkey.rs        # Global hotkey hook
│   │   ├── audio.rs         # WASAPI audio capture
│   │   ├── asr.rs           # Whisper integration
│   │   ├── ai_provider.rs   # LLM API client
│   │   ├── typing.rs        # SendInput / clipboard typing
│   │   ├── overlay.rs       # Overlay window management
│   │   ├── tray.rs          # System tray setup
│   │   ├── credentials.rs   # Credential Manager wrapper
│   │   ├── history.rs       # SQLite history storage
│   │   └── config.rs        # Settings management
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                 # Web frontend (React + TypeScript)
│   ├── App.tsx
│   ├── components/
│   │   ├── Overlay.tsx
│   │   ├── Settings.tsx
│   │   └── History.tsx
│   ├── hooks/
│   └── styles/
├── docs/
│   └── project/         # This documentation
├── models/              # Whisper model files (gitignored)
└── package.json
```
