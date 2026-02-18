# Architecture & Technology Decisions

## 1. High-Level Architecture

> **Current Status (Feb 2026):** Pure Rust CLI/system-tray app. No Tauri/UI layer. Dual-mode transcription (local Whisper + OpenAI API). Observability layer planned (Phase 4).

FluidVoice for Windows is a **native Rust desktop application** composed of four main layers:

```
┌──────────────────────────────────────────────────────┐
│            Presentation (System Tray)                │
│      tray-item · Audio Feedback (rodio)              │
├──────────────────────────────────────────────────────┤
│                  Application Core                    │
│   Hotkey Manager (rdev) · Pipeline Controller        │
│   Config (TOML) · Telemetry (stats.json)             │
├──────────────────────────────────────────────────────┤
│                  Service Layer                       │
│   Audio Capture (cpal) · Local ASR (whisper-rs)      │
│   Cloud ASR (OpenAI API) · Text Injection (enigo)    │
├──────────────────────────────────────────────────────┤
│            Observability (Phase 4 — Planned)         │
│   Structured Logging (tracing) · Crash Reports       │
│   Heartbeat · Diagnostics Export · Audio Metrics     │
├──────────────────────────────────────────────────────┤
│              Platform Integration                    │
│   WASAPI (via cpal) · Keyboard Sim · System Tray     │
│   Single Instance Lock                               │
└──────────────────────────────────────────────────────┘
```

### Component Descriptions

| Layer | Component | Responsibility | Status |
|-------|-----------|----------------|--------|
| **Presentation** | System Tray (`tray-item`) | App lifecycle, context menu (Quit) | ✅ Done |
| | Audio Feedback (`rodio`) | Beep on start/stop/error | ✅ Done |
| | Overlay Window | Visual recording/transcription feedback | ⬜ Not started |
| **Core** | Hotkey Manager (`rdev`) | Listens for Ctrl+Shift+V globally | ✅ Done |
| | Pipeline Controller | Orchestrates record → transcribe → type | ✅ Done |
| | Config (`toml`) | `config.toml` for hotkey, duration, API key | ✅ Done |
| | Telemetry | Local `stats.json` usage tracking | ✅ Done (basic) |
| **Services** | Audio Capture (`cpal`) | Records mic via WASAPI, resamples to 16kHz mono | ✅ Done |
| | Local ASR (`whisper-rs`) | Runs Whisper inference on audio buffer | ✅ Done |
| | Cloud ASR (`reqwest`) | Sends WAV to OpenAI Whisper API | ✅ Done |
| | Text Injection (`enigo`) | Injects text as keystrokes into focused window | ✅ Done |
| **Observability** | Structured Logging (`tracing`) | JSON file logging, timing instrumentation | ⬜ Phase 4 |
| | Crash Reporting (`sentry`) | Remote panic/error capture | ⬜ Phase 4 |
| | Heartbeat | Periodic health ping to backend | ⬜ Phase 4 |
| | Diagnostics Export | One-click zip of logs + config + system info | ⬜ Phase 4 |
| **Platform** | WASAPI / Win32 | Low-level Windows audio + input | ✅ Done |

---

## 2. Tech Stack (Current)

### Chosen: **Pure Rust (no Tauri, no frontend)**

> The original plan called for Tauri 2 + React. During MVP development, we chose to ship a pure Rust CLI/tray app for maximum speed-to-market. Tauri may be revisited for the Settings UI and Overlay in a future phase.

| Factor | Rationale |
|--------|-----------|
| **Speed to market** | Pure Rust CLI shipped in 5 days vs weeks with Tauri scaffolding |
| **Binary size** | ~8 MB release binary (stripped + LTO) |
| **System access** | Direct `cpal`, `rdev`, `enigo` — no IPC overhead |
| **ASR integration** | `whisper-rs` runs in-process |
| **Dual-mode builds** | Cargo feature flags: `--features local` or `--features openai` |

### Crates in Use

| Crate | Version | Purpose |
|-------|---------|---------|
| `cpal` | 0.15 | WASAPI audio capture |
| `whisper-rs` | 0.15.1 | Local Whisper inference (optional, `local` feature) |
| `reqwest` | 0.11 | OpenAI API calls (optional, `openai` feature) |
| `enigo` | 0.1 | Keyboard simulation / text injection |
| `rdev` | 0.5 | Global hotkey listener |
| `tray-item` | 0.10 | System tray icon + menu |
| `rodio` | 0.18 | Audio feedback (beeps) |
| `single-instance` | 0.3 | Prevent duplicate processes |
| `toml` | 0.8 | Config file parsing |
| `serde` / `serde_json` | 1.0 | Serialization |
| `hound` | 3.5 | WAV file encoding |
| `anyhow` | 1.0 | Error handling |
| `colored` | 2.1 | CLI colored output (debug mode) |

### Planned Crates (Phase 4 — Observability)

| Crate | Version | Purpose |
|-------|---------|---------|
| `tracing` | 0.1 | Structured logging |
| `tracing-subscriber` | 0.3 | JSON log output + filtering |
| `tracing-appender` | 0.2 | Daily log rotation to file |
| `dirs` | 5.0 | Resolve `%LOCALAPPDATA%` |
| `sentry` | 0.31 | Crash/panic reporting |
| `sysinfo` | 0.38 | System info for diagnostics |
| `self_update` | 0.27 | Auto-updater via GitHub Releases |

---

## 3. Key Component Design

### 3.1 Global Hotkey (rdev)

- Uses `rdev::listen()` in a background thread.
- Tracks `Ctrl` + `Shift` modifier state manually.
- Triggers pipeline on `V` keypress when both modifiers are held.
- Communicates to main thread via `mpsc::channel`.

### 3.2 Audio Capture (cpal)

- Uses WASAPI via `cpal` for microphone capture.
- Captures PCM audio at the device's native sample rate.
- Converts to mono + resamples to 16 kHz (Whisper's native rate) in software.
- Fixed recording duration (configurable via `config.toml`, default 5s).

### 3.3 ASR Engine (Dual-Mode via Feature Flags)

**Local mode** (`--features local`, default):
- `whisper-rs` wraps `whisper.cpp`.
- Model (`ggml-base.en.bin`) auto-downloaded on first run.
- Inference runs in-process on CPU.

**Cloud mode** (`--features openai`):
- `reqwest` sends WAV file to OpenAI Whisper API.
- Requires `openai_api_key` in `config.toml` or `.env`.

### 3.4 Text Injection (enigo)

- Uses `enigo` crate for cross-platform keyboard simulation.
- Types character-by-character with 10ms delay for stability.
- Handles special keys: `\n` → Enter, `\t` → Tab.

### 3.5 System Tray (tray-item)

- `tray-item` crate provides simple tray icon + context menu.
- Current menu: "Quit" only.
- Planned additions: "Export Diagnostics", "Check for Updates" (Phase 4/5).

### 3.6 Telemetry (Current — Basic)

- Local `stats.json` file tracks: `total_recordings`, `total_audio_seconds`, `errors_encountered`, `last_used`.
- No remote reporting yet — Phase 4 will add structured logging, Sentry, heartbeat.

---

## 4. Data Flow — Dictation Session

```
User presses Ctrl+Shift+V
       │
       ▼
┌─────────────┐
│ rdev listen  │── mpsc::send() ──▶ Main Loop
└─────────────┘                        │
                                       ▼
                                  play_start() beep
                                       │
                                       ▼
                                ┌──────────────┐
                                │ record_audio │  (cpal, WASAPI)
                                │ 5s capture   │
                                └──────┬───────┘
                                       │ f32 samples
                                       ▼
                                  to_mono() → resample() → 16kHz
                                       │
                        ┌──────────────┤──────────────┐
                        │ [local]      │              │ [openai]
                        ▼              │              ▼
                  ┌───────────┐        │       ┌───────────────┐
                  │whisper-rs │        │       │ OpenAI API    │
                  │ inference │        │       │ (reqwest POST)│
                  └─────┬─────┘        │       └──────┬────────┘
                        │              │              │
                        └──────────────┘──────────────┘
                                       │ transcript text
                                       ▼
                                ┌──────────────┐
                                │  type_text() │  (enigo)
                                │ char-by-char │
                                └──────┬───────┘
                                       │
                                       ▼
                                 play_stop() beep
                                 save_stats()
```

---

## 5. Directory Structure (Actual)

```
FluidVoice/
├── MVP/                         # The main application
│   ├── src/
│   │   ├── main.rs              # Entry point, pipeline, audio, hotkey
│   │   ├── config.rs            # AppConfig struct, TOML loading
│   │   ├── telemetry.rs         # UsageStats, stats.json read/write
│   │   ├── tray.rs              # System tray (tray-item)
│   │   ├── audio_feedback.rs    # Beep sounds (rodio)
│   │   ├── model.rs             # Whisper model download (local feature)
│   │   └── transcriber.rs       # LocalTranscriber (local feature)
│   ├── dist/                    # Distribution files
│   │   ├── config.toml          # Default config
│   │   └── config-openai.toml   # OpenAI config example
│   ├── Cargo.toml               # Dependencies + feature flags
│   └── README.md
├── docs/
│   └── project/                 # This documentation
├── roadmap_fv.md                # Active roadmap (Phases 1–5)
├── tasks_fv.md                  # Granular technical task checklist
└── WINDOWS_PORT_ANALYSIS.md     # Original feasibility study
```

---

## 6. Build Configuration

Two release builds via Cargo feature flags:

```powershell
# Local Whisper (default) — ships with model downloader
cargo build --release --features local

# OpenAI Cloud — requires API key in config
cargo build --release --features openai
```

Release profile (`Cargo.toml`):
```toml
[profile.release]
strip = true        # Strip symbols
opt-level = "z"     # Optimize for size
lto = true          # Link-Time Optimization
codegen-units = 1   # Max optimization
panic = "abort"     # Smaller binary
```
