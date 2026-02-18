# FluidVoice for Windows — Project Documentation

> Reimplementing [FluidVoice](https://github.com/nickstuart-net/FluidVoice) (macOS voice-to-text dictation with AI enhancement) as a **native Windows desktop application**.

---

## 🚀 Current Status: Phase 3 Complete → Phase 4 Next

The **Walking Skeleton MVP** is live and usable with **two build modes**:
- **Location**: [`../../MVP/`](../../MVP/)
- **Download**: See [`../../MVP/dist/`](../../MVP/dist/) for the standalone executable.
- **Features completed**:
  - ✅ Global Hotkey (`Ctrl+Shift+V`)
  - ✅ System Tray (background process)
  - ✅ Audio Feedback (beep on start/stop/error)
  - ✅ Local Whisper Transcription (`whisper-rs`, default)
  - ✅ OpenAI Cloud Transcription (feature flag)
  - ✅ Single Instance Lock
  - ✅ Basic telemetry (`stats.json`)
- **Next up**: Phase 4 — Observability & Monitoring (structured logging, crash reporting, diagnostics)

---

## Quick Context

FluidVoice is an open-source macOS app that provides global hotkey dictation, AI-enhanced transcription, and the ability to type transcribed text directly into any focused application. This project delivers an equivalent Windows experience — **not** a direct Swift port, but a **ground-up reimplementation** in Rust against Windows APIs.

See [`WINDOWS_PORT_ANALYSIS.md`](../../WINDOWS_PORT_ANALYSIS.md) for the original feasibility study.

---

## Documentation Index

| Document | Purpose |
|----------|---------|
| [PRD.md](PRD.md) | Product Requirements Document — features, user stories, scope |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Architecture decisions, tech stack, component design |
| [PLATFORM_MAPPING.md](PLATFORM_MAPPING.md) | macOS → Windows API mapping reference |
| [ASR_STRATEGY.md](ASR_STRATEGY.md) | Speech recognition strategy & engine comparison |
| [RISK_REGISTER.md](RISK_REGISTER.md) | Risks, mitigations, and open questions |
| [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md) | Dev environment setup & build instructions |
| [roadmap_fv.md](../../roadmap_fv.md) | Active development roadmap (Phases 1–5) |
| [tasks_fv.md](../../tasks_fv.md) | Granular technical task checklist |

---

## Project Principles

1. **Dictation-first** — "hold to record → release to transcribe → type into focused app" must work flawlessly before anything else.
2. **Privacy by default** — on-device ASR (Whisper) as the primary engine; cloud is opt-in.
3. **Native feel** — system tray, global hotkeys should feel like a first-class Windows citizen.
4. **Observable** — when deployed on customer PCs, the app should be debuggable from logs alone.
5. **Feature parity over time** — ship a solid dictation MVP, then iterate toward full FluidVoice feature parity.
