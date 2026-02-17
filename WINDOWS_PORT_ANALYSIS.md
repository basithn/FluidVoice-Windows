# FluidVoice → Windows: Port Analysis

This document analyzes the **FluidVoice** macOS app (open-source voice-to-text dictation with AI enhancement) and what it would take to build a similar version for **Windows**.

---

## 1. What FluidVoice Does (Summary)

- **Voice-to-text dictation** with global hotkey (e.g. hold a key to record, release to transcribe).
- **Multiple speech models**: Parakeet TDT v3/v2 (Apple Silicon CoreML), Apple Speech, Whisper (cross-platform).
- **AI enhancement**: optional post-processing via OpenAI, Groq, OpenRouter, Ollama, etc.
- **Smart typing**: types transcribed (or AI-rewritten) text into the **currently focused** app.
- **Command mode**: voice commands that trigger actions (e.g. “open Slack”, “next tab”).
- **Rewrite / Write mode**: select text → voice instruction → AI rewrites; or no selection → AI writes from voice.
- **Overlay UI**: “notch”-style overlay (DynamicNotchKit), bottom overlay, live preview.
- **Menu bar app**: runs from menu bar with settings, history, updates.
- **Auto-updates**: GitHub releases + in-app updater.
- **Secure storage**: API keys in macOS Keychain.

---

## 2. macOS-Only / Platform-Specific Parts

### 2.1 **Core stack (cannot be reused on Windows as-is)**

| Area | macOS usage | Windows equivalent / effort |
|------|-------------|-----------------------------|
| **Language & UI** | Swift, SwiftUI, AppKit | Full rewrite: C#/WinUI or C++/Win32, or Electron/TAURI + TypeScript, or Rust + Tauri. |
| **Build / IDE** | Xcode, SwiftPM | Visual Studio / MSBuild, or CMake + Ninja, or Node/npm (if Electron). |
| **App model** | `.app` bundle, NSApplication, menu bar | Win32/WinUI app, system tray, UWP or desktop. |

So: **you are not “porting” the Swift codebase**. You are **reimplementing the product** on Windows with a new stack.

---

### 2.2 **System integration (must be reimplemented per OS)**

| Feature | macOS implementation | Windows equivalent |
|--------|----------------------|--------------------|
| **Global hotkeys** | `CGEvent.tapCreate` (event tap) + Accessibility trust | Low-level keyboard hook: `SetWindowsHookEx(WH_KEYBOARD_LL)` or `RegisterHotKey`; admin or “run as” / UIPI considerations. |
| **“Type into any app”** | Accessibility: `AXUIElement`, focused element, + `CGEvent` unicode/key events to target PID | UI Automation (`IUIAutomation`) or accessibility APIs; synthetic input via `SendInput()` or UI Automation `SetValue`. Needs careful handling of elevated vs non-elevated apps. |
| **Get selected text** | `AXUIElementCopyAttributeValue(..., kAXSelectedTextAttribute)` | No single API: clipboard backup (Ctrl+C → read → restore), or UI Automation / accessibility on focused control. App-specific quirks. |
| **Active/frontmost app** | `NSWorkspace.shared.frontmostApplication` | `GetForegroundWindow()` → `GetWindowThreadProcessId()`; window title/app name via Win32 or UI Automation. |
| **Accessibility / trust** | `AXIsProcessTrusted()`, `AXIsProcessTrustedWithOptions` | No direct equivalent; app must run with sufficient privileges or use UI Automation; some features may require “run as administrator” or user trust. |
| **Clipboard** | `NSPasteboard.general` | `OpenClipboard` / `GetClipboardData` / `SetClipboardData` or .NET `Clipboard` API. |
| **Secure storage (API keys)** | Security.framework Keychain (`SecItemAdd`, `SecItemCopyMatching`) | DPAPI, or Windows Credential Manager, or .NET `ProtectedData` / `CredentialManager`. |

All of these exist on Windows but with **different APIs and quirks**; each needs a dedicated Windows implementation.

---

### 2.3 **Speech / audio pipeline**

| Component | macOS | Windows |
|-----------|--------|---------|
| **Parakeet TDT v3/v2** | FluidAudio (CoreML, Apple Silicon optimized) | **Not available.** No CoreML on Windows. Options: (1) Use **Whisper** (or other cross-platform model) as primary; (2) Port Parakeet to ONNX/DirectML or other Windows ML stack (large effort); (3) Use a Windows-native ASR (e.g. Windows Speech Recognition, cloud APIs). |
| **Apple Speech** | `SFSpeechRecognizer`, `Speech` framework | **N/A.** Use **Windows.Media.SpeechRecognition** (UWP) or **System.Speech** / **Microsoft.CognitiveServices.Speech** (SDK), or cloud (Azure, Google, etc.). |
| **Whisper** | SwiftWhisper (C++/Python lib wrapped) | Same model runs on Windows: use **whisper.cpp**, **faster-whisper**, or a .NET/Node binding. |
| **Audio capture** | AVFoundation (`AVCaptureDevice`, `AVAudioEngine`) | **WASAPI** (preferred) or **DirectSound** / **WinMM**; or .NET `NAudio`, or UWP `MediaCapture`. |
| **Microphone permission** | `AVCaptureDevice.requestAccess(for: .audio)` | **Settings → Privacy → Microphone**; app capability + runtime check (e.g. `MicrophonePermission` in UWP or custom check on desktop). |

So: **no FluidAudio or Apple Speech on Windows**. You need a **Windows-native ASR strategy** (Whisper + optional Windows Speech / cloud).

---

### 2.4 **UI and presentation**

| Feature | macOS | Windows |
|--------|--------|---------|
| **Main window** | SwiftUI `WindowGroup` + AppKit | WinUI 3 / WPF / Win32 window; or web (Electron/TAURI) + HTML/CSS. |
| **Menu bar** | `NSStatusItem` (menu bar extra) | **System tray** icon + context menu (Win32 `Shell_NotifyIcon` or WinUI/WPF APIs). |
| **Overlay / “notch”** | `DynamicNotchKit` (floating NSWindow, notch-style) | Custom **transparent, topmost, borderless** window; position at top-center or per-monitor. Implement from scratch (Win32 or WinUI). |
| **Escape to dismiss** | `NSEvent.addGlobalMonitorForEvents`, `addLocalMonitorForEvents` | Global keyboard hook (same as hotkey) or `GetAsyncKeyState` in a loop; or message-only window with raw input. |
| **Animations / theme** | SwiftUI + AppKit, dark theme | Replicate in your chosen UI stack (e.g. WinUI composition, or CSS if Electron). |

Overlay and “notch” behavior are **reimplemented** on Windows with a different UI framework.

---

### 2.5 **Other dependencies**

| Dependency | Role on macOS | Windows |
|------------|----------------|---------|
| **AppUpdater** | GitHub releases, download, replace app, relaunch | Implement similar flow: check GitHub API, download installer/zip, run installer or replace exe and restart (with helper process or scheduled task). |
| **PromiseKit** | Async flow | Use language-native async (C# `async/await`, JS Promises, Rust futures). |
| **PostHog (analytics)** | Product analytics | Same PostHog; use Windows SDK or HTTP API from your stack. |
| **DynamicNotchKit** | Notch overlay UI | No equivalent; custom overlay window. |
| **MediaRemoteAdapter** | Media control (e.g. now playing) | Optional; Windows has different media APIs (e.g. System Media Transport Controls, or app-specific). |
| **Keychain** | Secure API key storage | Replaced by **Credential Manager** or **DPAPI** (see above). |

---

## 3. What Can Be Reused (Conceptually or in Code)

- **Product design**: flows (dictation → preview → type; rewrite; command mode), settings (hotkeys, model choice, AI providers), history.
- **AI/LLM integration**: REST APIs (OpenAI, Groq, OpenRouter, etc.) are platform-agnostic; reuse prompts, model IDs, and logic in the new stack.
- **Whisper**: Same models (e.g. GGUF/ONNX); use a Windows-compatible Whisper runtime (whisper.cpp, faster-whisper, etc.).
- **Analytics**: Same events and PostHog (or equivalent) from the new app.
- **Networking, JSON, crypto**: Standard in any modern Windows stack (e.g. .NET, Node, Rust).

So: **architecture and product logic** can be reused; **all OS and UI code** must be rewritten for Windows.

---

## 4. Effort Overview (Rough)

| Workstream | Relative effort | Notes |
|------------|-----------------|--------|
| **Choose stack & bootstrap** | Small | e.g. C# + WinUI 3, or Electron + TypeScript, or Tauri + Rust/TS. |
| **Audio capture (WASAPI / NAudio)** | Medium | Well-documented; driver/permission issues possible. |
| **ASR on Windows** | Medium–High | Integrate Whisper (or similar); no Parakeet/FluidAudio. Optional: Windows Speech or cloud. |
| **Global hotkey + “type into any app”** | High | Hooks, UIPI, focus vs overlay; many edge cases. |
| **Selected text + active app** | Medium | UI Automation / accessibility; fallbacks (clipboard) and app-specific behavior. |
| **Overlay UI (notch-like + bottom)** | Medium | Topmost windows, transparency, positioning. |
| **System tray, settings, history** | Medium | Standard desktop patterns. |
| **Secure storage (API keys)** | Small | Credential Manager or DPAPI. |
| **Auto-updater** | Small–Medium | Download + replace/install + restart. |
| **AI providers, prompts, MCP/command mode** | Medium | Mostly logic; adapt to new language and APIs. |
| **Testing & polish** | High | Many apps, focus/typing edge cases, permissions. |

**Overall**: A **competent developer** familiar with Windows desktop development could deliver a **first usable Windows version** (dictation + type-into-app + one ASR engine + basic settings) in the **order of 3–6 months** full-time, depending on stack and scope. Feature parity with macOS (command mode, rewrite/write, overlays, all providers) adds more.

---

## 5. Recommended Tech Directions for Windows

1. **C# + WinUI 3 (or WPF)**  
   - Good fit for native Windows APIs (Win32, UI Automation, WASAPI via NAudio), system tray, and secure storage.  
   - No Swift reuse; full rewrite in C#.

2. **Electron + TypeScript**  
   - Reuse logic in JS/TS (AI, networking, state).  
   - Native modules (Node) or child processes for: audio capture, global hotkeys, `SendInput`/typing, and overlay (e.g. transparent window).  
   - Heavier runtime and more “glue” for system integration.

3. **Tauri + Rust (core) + web frontend**  
   - Small binary; system integration in Rust (hooks, WASAPI, UI Automation via winapi/rust bindings or COM).  
   - UI in HTML/JS or a Rust UI crate; more initial work for Rust side.

4. **ASR**  
   - **Primary**: Whisper via **whisper.cpp** (C++ lib, callable from C#/Rust/Node native addon) or **faster-whisper** (Python; could run as subprocess).  
   - **Optional**: Windows built-in speech (UWP or SDK) or cloud (Azure Speech, etc.) for lower latency or different UX.

---

## 6. Summary Table: macOS vs Windows

| Capability | macOS (current) | Windows (to build) |
|------------|------------------|---------------------|
| **Codebase** | Swift, SwiftUI, AppKit | New codebase (C#, Electron, Rust, etc.) |
| **Global hotkey** | CGEvent tap + AX trust | Keyboard hook / RegisterHotKey + permissions |
| **Type into focused app** | CGEvent + AX focus | SendInput / UI Automation + focus handling |
| **Selected text** | AX selected text | UI Automation or clipboard fallback |
| **ASR** | Parakeet (FluidAudio), Apple Speech, Whisper | Whisper (+ optional Windows/cloud ASR) |
| **Overlay / notch** | DynamicNotchKit | Custom topmost transparent window |
| **Menu bar** | NSStatusItem | System tray icon + menu |
| **Secure storage** | Keychain | Credential Manager / DPAPI |
| **Auto-update** | AppUpdater (GitHub) | Custom or library (e.g. Squirrel) |

---

## 7. Conclusion

Building a **similar version of FluidVoice for Windows** means:

- **Not** a direct port of the Swift app.
- **Yes** a **new Windows application** that replicates:
  - **Features**: global hotkey dictation, “type into any app”, rewrite/write/command modes, overlays, history, AI providers, secure API keys, updates.
- **Replace** every macOS-specific API (AppKit, ApplicationServices, Speech, Keychain, FluidAudio, DynamicNotchKit) with **Windows equivalents** (Win32, UI Automation, WASAPI, Credential Manager, Whisper or other ASR, custom overlay).
- **Reuse** product design, AI/LLM integrations, and (for ASR) Whisper-based pipelines, in a new language and framework.

Realistic effort: **several months** for a solid first release, with ongoing work for parity and polish. Choosing one clear stack (e.g. C# + WinUI + Whisper) and nailing “dictation → type into app” and permissions first will set a good foundation.
