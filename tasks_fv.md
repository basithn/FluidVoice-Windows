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

