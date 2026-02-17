# Development Guide

## 1. Prerequisites

### Required Software

| Tool | Version | Purpose |
|------|---------|---------|
| **Rust** | 1.75+ (stable) | Backend / core logic |
| **Node.js** | 20 LTS+ | Frontend build tooling |
| **npm** | 10+ | Package manager |
| **Visual Studio Build Tools** | 2022+ | MSVC compiler for Rust on Windows |
| **Git** | 2.40+ | Version control |

### Optional (for GPU acceleration)

| Tool | Version | Purpose |
|------|---------|---------|
| **CUDA Toolkit** | 12.x | NVIDIA GPU acceleration for Whisper |
| **Vulkan SDK** | 1.3+ | AMD/Intel/NVIDIA GPU acceleration |

### Installation Steps

#### 1. Install Rust

```powershell
# Download and run rustup
winget install Rustlang.Rustup
# Or visit https://rustup.rs

# Verify
rustc --version
cargo --version
```

#### 2. Install Node.js

```powershell
winget install OpenJS.NodeJS.LTS

# Verify
node --version
npm --version
```

#### 3. Install Visual Studio Build Tools

```powershell
winget install Microsoft.VisualStudio.2022.BuildTools
```

During installation, select:
- **"Desktop development with C++"** workload
- Windows 10/11 SDK

#### 4. Install Tauri CLI

```powershell
cargo install tauri-cli
```

---

## 2. Project Setup

### Clone the Repository

```powershell
git clone <repository-url>
cd FluidVoice
```

### Install Frontend Dependencies

```powershell
npm install
```

### Download a Whisper Model (for development)

```powershell
# Create models directory
mkdir models

# Download base.en model (~142 MB)
curl -L -o models/ggml-base.en.bin https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin
```

---

## 3. Running the App

### Development Mode

```powershell
cargo tauri dev
```

This will:
1. Start the Vite dev server (frontend with hot reload)
2. Build and launch the Tauri app (Rust backend)
3. Open the app window

### Production Build

```powershell
cargo tauri build
```

Output will be in `src-tauri/target/release/bundle/` (MSI and/or EXE installer).

---

## 4. Project Structure

```
FluidVoice/
├── src-tauri/                  # Rust backend
│   ├── src/
│   │   ├── main.rs             # Entry point, Tauri setup
│   │   ├── hotkey.rs           # Global keyboard hook
│   │   ├── audio.rs            # WASAPI microphone capture
│   │   ├── asr.rs              # Whisper inference
│   │   ├── ai_provider.rs      # LLM API client
│   │   ├── typing.rs           # SendInput / clipboard paste
│   │   ├── overlay.rs          # Overlay window management
│   │   ├── tray.rs             # System tray
│   │   ├── credentials.rs      # Windows Credential Manager
│   │   ├── history.rs          # SQLite storage
│   │   └── config.rs           # Settings read/write
│   ├── Cargo.toml
│   └── tauri.conf.json         # Tauri configuration
├── src/                        # Frontend (React + TypeScript)
│   ├── App.tsx
│   ├── components/
│   ├── hooks/
│   └── styles/
├── docs/
│   └── project/                # Project documentation (you are here)
├── models/                     # Whisper GGML models (gitignored)
├── package.json
├── tsconfig.json
└── vite.config.ts
```

---

## 5. Key Development Workflows

### Adding a Tauri Command (Rust → Frontend IPC)

1. Define the command in Rust:

```rust
// src-tauri/src/main.rs
#[tauri::command]
fn get_transcription_history() -> Vec<HistoryEntry> {
    history::get_recent(50)
}
```

2. Register it in the Tauri builder:

```rust
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_transcription_history])
```

3. Call from the frontend:

```typescript
import { invoke } from '@tauri-apps/api/core';

const history = await invoke<HistoryEntry[]>('get_transcription_history');
```

### Testing Audio Capture

```powershell
# Run a specific Rust test
cargo test -p fluidvoice -- audio::tests
```

### Running All Tests

```powershell
# Backend tests
cargo test -p fluidvoice

# Frontend tests
npm test
```

---

## 6. Coding Standards

### Rust

- Follow standard Rust conventions (`rustfmt`, `clippy`).
- Use `thiserror` for error types; avoid `unwrap()` in production paths.
- Async operations use `tokio` runtime.
- All Win32 API calls go through the `windows` crate, wrapped in safe Rust.

### TypeScript / React

- Use functional components with hooks.
- State management via React Context or Zustand (TBD).
- Styles in CSS Modules or vanilla CSS (no Tailwind unless decided otherwise).

### General

- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/).
- Feature branches off `main`; PRs required for merge.
- All new features need at least basic test coverage.

---

## 7. Debugging Tips

### Tauri DevTools

Press `F12` in the app window to open Chrome DevTools for the frontend.

### Rust Logging

```rust
use tracing::{info, warn, error};

info!("Recording started, device: {}", device_name);
```

Set log level via environment:

```powershell
$env:RUST_LOG = "debug"
cargo tauri dev
```

### Common Issues

| Issue | Solution |
|-------|---------|
| `whisper-rs` build fails | Ensure Visual Studio Build Tools (C++ workload) is installed |
| No audio capture | Check Windows Privacy → Microphone settings |
| Hotkey not working | Run app as Administrator if target app is elevated |
| Tauri window not transparent | Check `tauri.conf.json` has `"transparent": true` in window config |
| Model not found at runtime | Ensure model file is in `models/` and path is correct in config |

---

## 8. Useful Links

| Resource | URL |
|----------|-----|
| Tauri v2 Docs | https://v2.tauri.app |
| whisper.cpp | https://github.com/ggerganov/whisper.cpp |
| whisper-rs | https://github.com/tazz4843/whisper-rs |
| cpal (audio I/O) | https://github.com/RustAudio/cpal |
| windows crate | https://github.com/microsoft/windows-rs |
| Conventional Commits | https://www.conventionalcommits.org |
