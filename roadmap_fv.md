# FluidVoice Windows MVP Roadmap

**Status:** 🚧 In Progress
**Version:** Walking Skeleton
**Timeline:** February 2026

---

## 📅 Phase 1: The Walking Skeleton (Days 1-5)
**Goal:** Ship a working dirty MVP to validate core utility.
**Focus:** Functionality > Architecture. No Tauri. No UI. Pure Rust CLI.

### Day 1: Foundation & Audio
- [x] **Project Bootstrap**: Setup Rust, crates, git.
- [x] **Audio Engine**: Implement WASAPI capture via `cpal`.
- [x] **File Handling**: Temporary WAV file generation.
- [x] **Transcription**: OpenAI Whisper API integration.
- [x] **Verification**: "Record -> Save -> Transcribe" loop working.

### Day 2: The Loop
- [x] **Text Injection**: Implement `enigo` for keyboard input.
- [ ] **Clipboard Fallback**: Reliability mechanism for failed typing.
- [x] **End-to-End Test**: Speak -> Typed in Notepad.
- [ ] **Error Handling**: Graceful degradation (don't crash, report error).

### Day 3: Usability (Global Hotkey)
- [x] **Global Listener**: Implement `rdev` for Ctrl+Shift+V.
- [x] **State Management**: Handling multiple triggers, thread safety.
- [x] **Feedback**: Console outputs, simple sounds or notifications.
- [ ] **Dogfooding**: Developer uses it for the rest of the day.

### Day 4: Robustness & Configuration
- [x] **Configuration**: `config.toml` for hotkeys, API keys, audio settings.
- [x] **Telemetry**: Local `stats.json` usage tracking.
- [x] **Compatibility Check**: Verify against Chrome, Slack, VS Code.
- [x] **Cleanup**: `cargo check`, removal of unused deps.

### Day 5: Packaging & Release
- [x] **Build**: Release mode optimization.
- [x] **Documentation**: README with setup and troubleshooting.
- [x] **Distribution**: ZIP file with `.exe` and sample config.
- [x] **Release**: Github Release v0.1.0.

---

## 🎨 Phase 2: Usability Polish (Days 6-8)
**Goal:** Native background app experience.

### Day 6: System Tray & Window Management
- [x] **System Tray**: Icon, Context Menu (Quit).
- [x] **Hidden Console**: Run without visible window.
- [x] **Single Instance**: Prevent duplicate processes.

### Day 7: Audio Feedback & UX
- [x] **Sound Effects**: Beep on start/stop.
- [x] **Notification**: Audio Error Feedback (Low tone).

### Day 8: Visual Feedback (Optional)
- [ ] **Overlay**: Small popup when recording.

---

## 🛠 Phase 3: Local Inference & Polish (Week 3-4)
**Goal:** Remove dependencies and improve performance.

### Local AI (Removing OpenAI)
- [x] Investigate `whisper.rs` or ONNX Runtime.
- [x] Ship optional "Offline Mode" (download model).
- [ ] Benchmark Local vs Cloud latency (Parked).
- [ ] Tuning & Resampling Quality (Parked).

### UX Polish
- [ ] System Tray Icon (status indicator).
- [ ] Visual Overlay (recording state).

---

## 🔍 Phase 4: Observability & Monitoring (Week 5-6)
**Goal:** Stop flying blind on customer machines. Diagnose any issue from logs alone.

### Week 5: Structured Logging & Crash Reporting
- [ ] Replace all `println!`/`eprintln!` with `tracing` crate (JSON structured logs).
- [ ] Log to file at `%LOCALAPPDATA%\FluidVoice\logs\` with daily rotation.
- [ ] Add timing instrumentation to `record_audio()`, `transcribe()`, `type_text()`.
- [ ] Move `stats.json` to `%LOCALAPPDATA%\FluidVoice\` (fix CWD problem).
- [ ] Enrich `stats.json`: add `app_version`, `os_version`, `machine_id`, `last_error_message`, `avg_transcription_latency_ms`.
- [ ] Integrate Sentry (`sentry = "0.31"`) for remote crash/panic reporting.

### Week 6: Heartbeat & Diagnostics
- [ ] Implement heartbeat: periodic HTTP POST with `{ machine_id, version, stats }`.
- [ ] Add "Export Diagnostics" to system tray menu (zip logs + config + system info).
- [ ] Add audio quality metrics: RMS volume, peak amplitude, silence ratio.
- [ ] Add `telemetry_enabled` opt-out toggle in `config.toml`.

---

## 📡 Phase 5: Fleet Operations (Month 3+)
**Goal:** Proactive fleet management — updates, accuracy, dashboard.

### Auto-Updates
- [ ] Integrate `self_update = "0.27"` for GitHub Releases auto-update.
- [ ] Version reporting in heartbeat telemetry.

### Accuracy Intelligence
- [ ] Log Whisper confidence scores (`exp(avg_logprob)`).
- [ ] Estimate accuracy via correction detection (post-injection backspace tracking).

### Fleet Dashboard
- [ ] Deploy backend (Supabase/Cloudflare Workers) for heartbeat ingestion.
- [ ] Build dashboard: active machines, version distribution, error rates, latency P50/P90.

---

## 📈 Success Metrics (MVP)
- **Latency:** < 3 seconds (Record End -> Text Appears).
- **Accuracy:** > 90% for clear English.
- **Reliability:** 0 Crashes during normal operation.
- **Usage:** Beta users type > 500 words/day using tool.

## 📈 Success Metrics (Observability — Phase 4+)
- **Debuggability:** Any customer issue diagnosable from log files alone.
- **Crash Visibility:** 100% of panics reported to Sentry within 24 hours.
- **Fleet Awareness:** Know which version each customer is running.
- **MTTR:** Mean-time-to-resolution < 1 hour for common issues (bad mic, config error).
- **Privacy:** Zero transcripts or PII transmitted. Anonymous IDs only.
