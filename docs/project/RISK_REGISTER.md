# Risk Register

This document tracks known risks, their likelihood, potential impact, and mitigations. Review and update regularly as the project progresses.

---

## Risk Matrix Key

| Likelihood | Impact | Severity |
|------------|--------|----------|
| High / High | → | 🔴 Critical |
| High / Medium or Medium / High | → | 🟠 High |
| Medium / Medium | → | 🟡 Medium |
| Low / any or any / Low | → | 🟢 Low |

---

## Active Risks

### R-01 🟠 `SendInput()` fails in elevated / UWP apps

| Field | Detail |
|-------|--------|
| **Description** | `SendInput()` cannot inject keystrokes into windows running at a higher integrity level (e.g. Task Manager, UAC prompts) or certain UWP sandboxed apps. |
| **Likelihood** | High |
| **Impact** | Medium — affects a subset of apps, not core use case |
| **Mitigation** | (1) Detect elevated target and warn user. (2) Offer clipboard-paste fallback (`Ctrl+V`). (3) Optionally run the app as Administrator. |
| **Owner** | TBD |
| **Status** | Open |

---

### R-02 🟠 Whisper latency on low-end hardware

| Field | Detail |
|-------|--------|
| **Description** | Whisper `base` model on older CPUs (e.g. dual-core, pre-AVX2) may produce unacceptable transcription latency (>5s for short clips). |
| **Likelihood** | Medium |
| **Impact** | High — poor UX makes the app unusable |
| **Mitigation** | (1) Default to `tiny` model on low-spec hardware, with detection at startup. (2) Offer GPU acceleration. (3) Show progress indicator during inference. (4) Set minimum hardware requirements. |
| **Owner** | TBD |
| **Status** | Open |

---

### R-03 🟡 "Get selected text" unreliable across apps

| Field | Detail |
|-------|--------|
| **Description** | The clipboard-copy technique (`Ctrl+C` → read → restore) for getting selected text is fragile: some apps don't support `Ctrl+C`, clipboard restore may race, and some apps modify clipboard content. |
| **Likelihood** | Medium |
| **Impact** | Medium — affects Rewrite mode, not core dictation |
| **Mitigation** | (1) Use UI Automation `IUIAutomation` where available. (2) Implement app-specific handlers for known problematic apps. (3) Add a timeout and graceful fallback if clipboard is not updated. |
| **Owner** | TBD |
| **Status** | Open |

---

### R-04 🟡 whisper.cpp / whisper-rs breaking changes

| Field | Detail |
|-------|--------|
| **Description** | `whisper.cpp` is under active development; API changes could break `whisper-rs` bindings. |
| **Likelihood** | High (Occurred v0.12 -> v0.15) |
| **Impact** | Medium — required code refactor |
| **Mitigation** | (1) Pin `whisper-rs` version. (2) Monitor releases. (3) **Status:** Mitigated in MVP (Pinned v0.15). |
| **Owner** | Basit |
| **Status** | **Monitoring** |

---

### R-05 🟢 Antivirus false positives on keyboard hooks

| Field | Detail |
|-------|--------|
| **Description** | Low-level keyboard hooks (`WH_KEYBOARD_LL`) can trigger antivirus / endpoint protection alerts (flagged as keylogger behavior). |
| **Likelihood** | Low |
| **Impact** | Medium — app may be quarantined or blocked |
| **Mitigation** | (1) Code-sign the executable with a trusted certificate. (2) Submit to major AV vendors for whitelisting. (3) Document in user guide how to add exception. |
| **Owner** | TBD |
| **Status** | Open |

---

### R-06 🟢 Microphone permission issues on Windows 10/11

| Field | Detail |
|-------|--------|
| **Description** | Windows 10/11 has per-app microphone permissions under Settings → Privacy. Desktop apps may be blocked if the global toggle or per-app toggle is off. |
| **Likelihood** | Low |
| **Impact** | High — no audio capture means no transcription |
| **Mitigation** | (1) Check microphone permission at startup and guide user to enable it. (2) Show a clear error message with a deep-link to Windows Privacy settings. |
| **Owner** | TBD |
| **Status** | Open |

---

### R-07 🟡 Tauri v2 maturity for advanced window features

| Field | Detail |
|-------|--------|
| **Description** | Tauri v2 is relatively new; advanced features like transparent/click-through overlay windows, multi-monitor positioning, and tray behaviors may have bugs or missing APIs. |
| **Likelihood** | Medium |
| **Impact** | Medium — may need workarounds or direct Win32 fallbacks |
| **Mitigation** | (1) Prototype overlay window early (Phase 1). (2) Be prepared to use raw Win32 APIs via the `windows` crate for overlay if Tauri APIs are insufficient. (3) Engage with Tauri community / issues. |
| **Owner** | TBD |
| **Status** | Open |

---

## Open Questions

| # | Question | Decision Required By | Notes |
|---|----------|---------------------|-------|
| Q-1 | Should the app require "Run as Administrator" by default? | Phase 1 | Trade-off: broader app compatibility vs. security concerns |
| Q-2 | Should we support Windows 10 or target Windows 11-only? | Phase 1 | Win 10 has larger market share but drops WinUI 3 features |
| Q-3 | How to handle enterprise environments with AppLocker / Group Policy? | Phase 4 | May block hooks, unsigned exes, or tray icons |
| Q-4 | Is code-signing certificate needed before public release? | Phase 4 | Required for SmartScreen trust + AV whitelist |
