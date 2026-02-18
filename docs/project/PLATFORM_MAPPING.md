# Platform API Mapping: macOS → Windows

This document is a reference for every macOS-specific API/framework used by FluidVoice and its Windows equivalent. Use this during implementation to quickly identify the right Windows API for each capability.

---

## 1. Application Model & UI Framework

| Capability | macOS API | Windows Equivalent | Rust Crate / Library |
|------------|-----------|-------------------|---------------------|
| Application bootstrap | `NSApplication`, SwiftUI `@main` | Win32 `WinMain` / Tauri app | `tauri` |
| Main window | SwiftUI `WindowGroup` | Tauri `WebviewWindow` | `tauri` |
| Menu bar app | `NSStatusItem` | `Shell_NotifyIcon` (system tray) | `tauri::tray` |
| Overlay / notch | `DynamicNotchKit` (floating `NSWindow`) | Topmost, transparent, borderless window | `tauri::WebviewWindow` with flags |
| Settings window | SwiftUI `Settings` scene | Web-based settings panel | React (Tauri frontend) |
| Animations / theme | SwiftUI animations | CSS animations / Framer Motion | Frontend framework |

---

## 2. Input & Accessibility

| Capability | macOS API | Windows Equivalent | Rust Crate / Library |
|------------|-----------|-------------------|---------------------|
| Global hotkey | `CGEvent.tapCreate` + `AXIsProcessTrusted` | `SetWindowsHookEx(WH_KEYBOARD_LL)` | `windows` crate |
| Configurable hotkey | Custom event tap filter | `RegisterHotKey` (for simple combos) | `windows` crate |
| Type into focused app | `CGEvent` key events with target PID | `SendInput()` with `KEYBDINPUT` | `windows` crate |
| Unicode text input | `CGEvent(keyboardEventSource:)` with unicode | `SendInput()` + `KEYEVENTF_UNICODE` | `windows` crate |
| Get selected text | `AXUIElementCopyAttributeValue(kAXSelectedTextAttribute)` | Clipboard backup: `Ctrl+C` → read → restore | Manual orchestration |
| Get focused element | `AXUIElement` focused element query | `IUIAutomation::GetFocusedElement` | `windows` crate (COM) |
| Frontmost application | `NSWorkspace.shared.frontmostApplication` | `GetForegroundWindow()` → `GetWindowThreadProcessId()` | `windows` crate |
| Accessibility trust | `AXIsProcessTrusted()` | No direct equivalent; may need admin privileges | N/A |
| Escape key monitoring | `NSEvent.addGlobalMonitorForEvents` | Same LL keyboard hook or `GetAsyncKeyState` | `windows` crate |

---

## 3. Audio & Speech

| Capability | macOS API | Windows Equivalent | Rust Crate / Library |
|------------|-----------|-------------------|---------------------|
| Audio capture | `AVCaptureDevice`, `AVAudioEngine` | **WASAPI** (preferred) | `cpal` or `wasapi` |
| Microphone permission | `AVCaptureDevice.requestAccess(for: .audio)` | Settings → Privacy → Microphone; runtime check | OS settings |
| Parakeet ASR (CoreML) | FluidAudio (Apple Silicon, CoreML) | **Not available**; use Whisper instead | `whisper-rs` |
| Apple Speech | `SFSpeechRecognizer` | Windows Speech Recognition / Azure Speech | N/A or `azure-speech-sdk` |
| Whisper ASR | SwiftWhisper (C++ bridge) | `whisper.cpp` via `whisper-rs` | `whisper-rs` |
| Audio device enumeration | `AVCaptureDevice.DiscoverySession` | WASAPI `IMMDeviceEnumerator` | `cpal` |

---

## 4. Storage & Security

| Capability | macOS API | Windows Equivalent | Rust Crate / Library |
|------------|-----------|-------------------|---------------------|
| Secure key storage | Keychain (`SecItemAdd`, `SecItemCopyMatching`) | Windows Credential Manager (`CredRead`, `CredWrite`) | `windows` crate |
| App settings | `UserDefaults` or plist | JSON file in `%APPDATA%\FluidVoice\` | `serde_json`, `dirs` |
| Database (history) | SQLite or SwiftData | SQLite | `rusqlite` or `sqlx` |
| Clipboard | `NSPasteboard.general` | `OpenClipboard` / `GetClipboardData` / `SetClipboardData` | `clipboard-win` or `arboard` |

---

## 5. Networking & Distribution

| Capability | macOS API | Windows Equivalent | Rust Crate / Library |
|------------|-----------|-------------------|---------------------|
| HTTP requests | `URLSession` | `reqwest` (Rust) or `fetch` (frontend) | `reqwest` |
| Auto-update | AppUpdater (GitHub releases) | `self_update` crate (GitHub Releases) | `self_update` |
| App packaging | `.app` bundle, `.dmg` | `.exe` + ZIP (manual), installer later | Manual / future |
| Analytics | PostHog (Swift SDK) | PostHog (REST API) or custom HTTP endpoint | `reqwest` |
| Crash reporting | Sentry (Swift SDK) | Sentry (Rust SDK) + Breakpad | `sentry` |
| Structured logging | `os_log` / CocoaLumberjack | `tracing` + `tracing-appender` (JSON to file) | `tracing`, `tracing-subscriber`, `tracing-appender` |
| System diagnostics | `sysctl`, `ProcessInfo` | `sysinfo` crate | `sysinfo` |

---

## 6. Miscellaneous

| Capability | macOS API | Windows Equivalent | Rust Crate / Library |
|------------|-----------|-------------------|---------------------|
| Media control | `MediaRemoteAdapter` (Now Playing) | System Media Transport Controls | Optional |
| Async patterns | PromiseKit / Swift async-await | Rust `async`/`await` + Tokio | `tokio` |
| Process management | `NSWorkspace`, `NSRunningApplication` | `CreateToolhelp32Snapshot`, `EnumProcesses` | `windows` crate, `sysinfo` |
| Notifications | `NSUserNotification` / `UNUserNotificationCenter` | Win32 `Shell_NotifyIcon` balloon or WinRT toast | `tauri::notification` |

---

## Quick Reference: Rust Crates

| Crate | Purpose |
|-------|---------|
| `windows` | Win32 / COM / WinRT bindings |
| `whisper-rs` | Whisper.cpp Rust bindings |
| `cpal` | Cross-platform audio I/O (WASAPI on Windows) |
| `enigo` | Keyboard simulation / text injection |
| `rdev` | Global hotkey listener |
| `reqwest` | HTTP client |
| `tray-item` | System tray icon + menu |
| `rodio` | Audio playback (beeps) |
| `single-instance` | Prevent duplicate processes |
| `serde` / `serde_json` | Serialization |
| `toml` | Config file parsing |
| `tokio` | Async runtime |
| `dirs` | Platform standard directories |
| `tracing` | Structured logging / diagnostics |
| `tracing-subscriber` | JSON log output + filtering |
| `tracing-appender` | Daily log rotation to file |
| `sentry` | Crash reporting |
| `sysinfo` | System information (diagnostics) |
| `self_update` | Auto-updater via GitHub Releases |
