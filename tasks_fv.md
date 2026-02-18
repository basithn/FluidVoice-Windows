# FluidVoice MVP Technical Tasks

## Step 1: Bootstrap Project
- [x] **Initialize Rust Project**
  - Run `cargo new fluidvoice-mvp --bin`
  - Update `Cargo.toml` with `[package]` metadata (version 0.1.0, edition 2021)
- [x] **Configure Dependencies**
  - [x] `anyhow = "1.0"` (Error handling)
  - [x] `cpal = "0.15"` (Audio WASAPI)
  - [x] `hound = "3.5"` (WAV encoding)
  - [x] `reqwest = { version = "0.11", features = ["json", "multipart", "blocking"] }` (API)
  - [x] `serde = { version = "1.0", features = ["derive"] }` & `serde_json`
  - [x] `tokio = { version = "1.35", features = ["rt-multi-thread", "macros"] }`
  - [x] `enigo = "0.1"` (Input simulation - note version 0.1.x)
  - [x] `rdev = "0.5"` (Global hotkeys)
  - [x] `colored = "2.1"` (CLI UX)
  - [x] `dotenv = "0.15"` (Secrets)
  - [ ] `tempfile = "3.8"` (Cleanup)
  - [ ] `toml = "0.8"` (Configuration)
  - [ ] `clipboard = "0.5"` (Fallback)
  - [ ] `notify-rust = "4.10"` (Feedback)
  - [ ] `ctrlc = "3.4"` (Graceful shutdown)
- [x] **Setup Git**
  - [x] Create `.gitignore` (target/, .env, *.wav)
  - [x] Create `.env` template (`OPENAI_API_KEY=`)

## Step 2: Day 1 - Audio & Transcription Layer
- [x] **Device Enumeration**
  - Implement `list_audio_devices()` to print available input devices.
  - Handle WASAPI device names and indices.
- [x] **Audio Capture Core**
  - Implement `record_audio(duration)` using `cpal`.
  - [x] Handle Sample Rate (Project to 16kHz or resample).
  - [x] Handle Channels (Stereo -> Mono mixdown or select chan 1).
  - [x] Implement buffer collection (f32 or i16 normalization).
  - [x] Add CLI progress indicator ("Recording... 3s").
- [x] **WAV Export**
  - Implement `save_to_wav()` using `hound`.
  - Ensure spec: 16kHz, 16-bit, Mono (Whisper requirement).
  - Use `tempfile` to avoid garbage accumulation. (Used logic, not crate yet)
- [x] **OpenAI Integration**
  - Implement `transcribe(path)` using `reqwest::blocking`.
  - Load API key from env/config.
  - Handle API errors (401, 429) gracefully.
  - Parse JSON response for `text` field.
- [x] **Verification (Day 1)**
  - [x] Run `cargo run`.
  - [x] Verify recording saves valid WAV.
  - [x] Verify transcript prints to console.
  - [x] Check latency <3s.

## Step 3: Day 2 - Text Injection & Feedback
- [x] **Typing Engine**
  - Implement `type_text(string)` using `enigo`.
  - [x] Implement char-by-char delay (10ms) for stability.
  - [x] Handle special keys (`\n` -> Enter, `\t` -> Tab).
- [ ] **Clipboard Fallback**
  - Wrap typing in `Result`. On error, copy text to clipboard.
  - Notify user: "Typing failed, copied to clipboard."
- [x] **Application Testing**
  - Test injection in: Notepad, Chrome, VS Code.
  - Document quirks in non-working apps (Admin/Games).

## Step 4: Day 3 - Global Hotkey & Wrapper
- [x] **Hotkey Listener**
  - Implement `rdev::listen` loop in a separate thread.
  - State tracking for `Ctrl` + `Shift` modifiers.
  - Trigger `handle_recording()` on `V` key press.
  - Ensure non-blocking main loop.
- [x] **Visual/Audio Feedback**
  - Add print statements for state changes (Listening, Recording, Processing).
  - (Optional) System notification on start/finish.
- [x] **Concurrency**
  - Use `std::sync::mpsc` or similar to handle hotkey events -> code execution.
  - Prevent multiple recordings triggering simultaneously.

## Step 5: Day 4 - Configuration & Polish
- [x] **Config System**
  - Define `Config` struct (Hotkeys, Audio settings, API settings).
  - Implement `load_config()` reading `config.toml`.
  - Generate default `config.toml` if missing.
- [x] **Error Handling & Logging**
  - Wrap main logic in catch-all to prevent crashes.
  - Write generic errors to `error.log`. (Logged to telemetry instead)
  - Implement `ctrlc` handler for cleanup.
- [x] **Telemetry (Opt-in)**
  - Create `UsageStats` struct.
  - Save/Load `stats.json` (Count, Duration, Errors).
  - Print session summary on exit.

## Step 6: Final Verification
- [x] **Performance Check**
  - [x] Real-time factor > 5x.
  - [x] Latency < 3s.
- [x] **Stability Check**
  - [x] Hammer test: Rapid hotkey presses.
  - [x] Long run test: Leave running 1 hour.
- [x] **Packaging**
  - Run `cargo build --release`.
  - Create standard distribution folder.

## Step 7: Phase 2 - Usability Polish
- [x] **System Tray Integration**
  - [x] Add `tray-item` dependency.
  - [x] Implement Tray Menu with "Quit" option.
  - [x] Ensure app runs in background loop (no visible console in Release).
- [x] **Audio Feedback**
  - [x] Add `rodio` dependency.
  - [x] Implement procedural audio generation (Sine wave beeps).
  - [x] Play "High Beep" on Start, "Double Beep" on Stop.
- [x] **Window Management**
  - [x] Configure `#![windows_subsystem = "windows"]` for Release build to hide console.
  - [x] Implement `single-instance` check to prevent duplicates.

## Step 8: Phase 3 - Local Whisper (Implemented)
- [x] **Engine Replacement**
  - [x] Replace `reqwest` (OpenAI) with `whisper` (C++ bindings).
  - [x] Implement model downloader (ggml-base.en.bin).
  - [x] Implement `src/transcriber.rs` for local inference.
- [ ] **Performance Tuning** (Parked)
  - [ ] Benchmark transcription time vs. Cloud.
  - [ ] Optimize model loading (keep loaded in memory).
- [x] **Offline Capability**
  - [x] Verify functionality with network disabled.

## Step 9: Quality Tuning (Parked)
- [ ] **Resampling**
  - [ ] Upgrade to `rubato` for better 48kHz -> 16kHz conversion.
- [ ] **Model Selection**
  - [ ] Support `small.en` or `medium.en` for better accuracy.

## Step 10: Structured Logging (Phase 4 — Observability)
- [ ] **Add Logging Dependencies**
  - [ ] `tracing = "0.1"` (structured logging facade)
  - [ ] `tracing-subscriber = { version = "0.3", features = ["json", "env-filter"] }` (JSON output)
  - [ ] `tracing-appender = "0.2"` (file rotation)
  - [ ] `dirs = "5.0"` (resolve `%LOCALAPPDATA%` path cross-platform)
- [ ] **Initialize Logger in `main()`**
  - [ ] Create log dir at `%LOCALAPPDATA%\FluidVoice\logs\`.
  - [ ] Configure `tracing_appender::rolling::daily()` for daily rotation.
  - [ ] Use `tracing_subscriber::fmt().json()` for structured JSON output.
  - [ ] Store `_guard` to ensure logs flush on shutdown.
- [ ] **Replace All Console Output**
  - [ ] Replace every `println!()` → `tracing::info!()` in `main.rs`.
  - [ ] Replace every `eprintln!()` → `tracing::error!()` in `main.rs`.
  - [ ] Replace prints in `config.rs`, `telemetry.rs`, `tray.rs`, `audio_feedback.rs`.
  - [ ] Replace prints in `model.rs` and `transcriber.rs` (local feature).
- [ ] **Add Timing Instrumentation**
  - [ ] Add `#[tracing::instrument]` to `record_audio()`.
  - [ ] Add `#[tracing::instrument]` to `run_pipeline()` (both local/openai variants).
  - [ ] Add `#[tracing::instrument]` to `type_text()`.
  - [ ] Log `transcription_latency_ms` as a numeric field.
  - [ ] Log `pipeline_total_ms` from hotkey to text-injected.
- [ ] **Fix stats.json Location**
  - [ ] Move `stats.json` read/write to `%LOCALAPPDATA%\FluidVoice\stats.json`.
  - [ ] Handle migration: if old CWD `stats.json` exists, move it to new location.
- [ ] **Enrich UsageStats Struct**
  - [ ] Add `app_version: String` (from `env!("CARGO_PKG_VERSION")`).
  - [ ] Add `os_version: String` (via `sysinfo` or Windows API).
  - [ ] Add `machine_id: String` (generate random UUID on first run, persist).
  - [ ] Add `last_error_message: Option<String>` (store actual error text).
  - [ ] Add `avg_transcription_latency_ms: f64` (rolling average).
  - [ ] Add `transcription_mode: String` ("local" or "openai").
- [ ] **Verification**
  - [ ] Build release mode → verify no console window appears.
  - [ ] Trigger hotkey → verify `fluidvoice.log` created with JSON entries.
  - [ ] Trigger an error → verify error logged with context (not just counter).
  - [ ] Check `stats.json` created in `%LOCALAPPDATA%\FluidVoice\`.

## Step 11: Crash Reporting (Phase 4 — Observability)
- [ ] **Add Sentry Dependencies**
  - [ ] `sentry = "0.31"` (core SDK, captures panics automatically).
  - [ ] `sentry-contrib-breakpad = "0.9"` (optional: native minidump on hard crash).
- [ ] **Initialize Sentry in `main()`**
  - [ ] Call `sentry::init()` with DSN, `release: sentry::release_name!()`.
  - [ ] Store `_sentry_guard` to flush on shutdown.
  - [ ] Ensure `panic = "abort"` in `[profile.release]` (already set) works with Sentry.
- [ ] **Add Error Context**
  - [ ] In pipeline error handler, call `sentry::capture_error()` with the `anyhow::Error`.
  - [ ] Attach tags: `mode` (local/openai), `app_version`, `machine_id`.
- [ ] **Privacy**
  - [ ] Verify no transcript text appears in Sentry events.
  - [ ] Verify no API key appears in Sentry events.
  - [ ] Only anonymous `machine_id` used as user identifier.
- [ ] **Verification**
  - [ ] Force a panic → verify event appears in Sentry dashboard.
  - [ ] Simulate API error → verify error captured with tags.
  - [ ] Disable network → verify crash report queued and sent on next launch.

## Step 12: Heartbeat & Diagnostics (Phase 4 — Observability)
- [ ] **Config: Telemetry Opt-Out**
  - [ ] Add `telemetry_enabled: bool` field to `AppConfig` (default `true`).
  - [ ] Gate all remote telemetry (heartbeat, Sentry) behind this flag.
  - [ ] Document in config.toml comments: what data is sent.
- [ ] **Heartbeat Implementation**
  - [ ] Define `HeartbeatPayload` struct: `machine_id`, `app_version`, `os_version`, `mode`, `total_recordings`, `errors`, `uptime_minutes`, `avg_latency_ms`.
  - [ ] Send HTTP POST on startup (fire-and-forget, non-blocking).
  - [ ] Send periodic heartbeat every 60 minutes via background thread.
  - [ ] Gracefully handle network failures (log warning, don't retry aggressively).
- [ ] **Audio Quality Metrics**
  - [ ] Implement `audio_quality(samples) -> AudioMetrics` function.
  - [ ] Compute: `rms_volume`, `peak_amplitude`, `silence_ratio`.
  - [ ] Log metrics with `tracing::info!()` before each transcription.
  - [ ] Add to `stats.json`: `last_audio_rms`, `last_audio_peak`.
- [ ] **Diagnostic Export**
  - [ ] Add `sysinfo = "0.38"` dependency.
  - [ ] Add "Export Diagnostics" item to system tray context menu.
  - [ ] On click, collect: log files (last 7 days), `stats.json`, `config.toml` (mask API key), system info (OS, CPU, RAM, audio devices via `cpal`).
  - [ ] Zip into `FluidVoice-diagnostics-YYYYMMDD.zip` on Desktop.
  - [ ] Show Windows notification: "Diagnostics saved to Desktop".
- [ ] **Verification**
  - [ ] Run app → verify heartbeat POST sent on startup (check server/logs).
  - [ ] Run app offline → verify no crash, warning logged.
  - [ ] Click "Export Diagnostics" → verify zip created with all expected files.
  - [ ] Verify API key is masked in exported config.

## Step 13: Fleet Operations (Phase 5)
- [ ] **Auto-Updater**
  - [ ] Add `self_update = "0.27"` dependency.
  - [ ] Implement update check on startup (compare `CARGO_PKG_VERSION` with latest GitHub Release).
  - [ ] If update available, log it and show system tray notification.
  - [ ] Add "Check for Updates" to system tray menu.
  - [ ] Handle update failures gracefully (log, don't crash).
- [ ] **Accuracy Estimation**
  - [ ] Extract Whisper confidence scores (local: from `whisper-rs` segment results; API: from response if available).
  - [ ] Log confidence score per transcription event.
  - [ ] (Optional, privacy-sensitive) Track post-injection backspace count as correction proxy.
- [ ] **Fleet Dashboard Backend**
  - [ ] Deploy heartbeat ingestion API (Supabase / Cloudflare Workers + D1).
  - [ ] Store: `machine_id`, `version`, `timestamp`, `stats_snapshot`.
  - [ ] Build simple web dashboard: active machines, version pie chart, error rate timeline.
- [ ] **Verification**
  - [ ] Publish a test GitHub Release → verify auto-update detects it.
  - [ ] Verify confidence scores logged for both local and openai modes.
  - [ ] Verify dashboard shows connected machines after heartbeat.
