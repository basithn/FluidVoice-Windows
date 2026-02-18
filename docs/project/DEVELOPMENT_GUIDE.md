# Development Guide

## 1. Prerequisites

### Required Software

| Tool | Version | Purpose |
|------|---------|---------|
| **Rust** | 1.75+ (stable) | Application language |
| **Visual Studio Build Tools** | 2022+ | MSVC compiler (required for `whisper-rs` C++ bindings) |
| **Git** | 2.40+ | Version control |
| **CMake** | 3.25+ | Required by `whisper-rs` build process |

> **Note:** Node.js / npm are **not needed** for the current MVP. The app is pure Rust with no frontend.

### Optional (for GPU acceleration)

| Tool | Version | Purpose |
|------|---------|---------|
| **CUDA Toolkit** | 12.x | NVIDIA GPU acceleration for Whisper |

### Installation Steps

#### 1. Install Rust

```powershell
# Download and run rustup
winget install Rustlang.Rustup

# Verify
rustc --version
cargo --version
```

#### 2. Install Visual Studio Build Tools

```powershell
winget install Microsoft.VisualStudio.2022.BuildTools
```

During installation, select:
- **"Desktop development with C++"** workload
- Windows 10/11 SDK

#### 3. Install CMake

```powershell
winget install Kitware.CMake
```

---

## 2. Project Setup

### Clone the Repository

```powershell
git clone <repository-url>
cd FluidVoice
```

### Configure API Key (OpenAI build only)

```powershell
# Copy example config
cp MVP/dist/config-openai.toml.example MVP/dist/config-openai.toml

# Edit with your API key
notepad MVP/dist/config-openai.toml
```

---

## 3. Building & Running

### Development Mode

```powershell
cd MVP

# Run with local Whisper (default)
cargo run

# Run with OpenAI cloud mode
cargo run --no-default-features --features openai
```

On first run (local mode), the app will automatically download `ggml-base.en.bin` (~142 MB) from Hugging Face.

### Production Build

```powershell
cd MVP

# Local Whisper build (default)
cargo build --release

# OpenAI cloud build
cargo build --release --no-default-features --features openai
```

Output: `MVP/target/release/fluidvoice-mvp.exe`

### Distribution Package

The `MVP/dist/` folder contains the distribution template:
```
dist/
├── fluidvoice-mvp.exe      # Copy from target/release/
├── config.toml              # Default config (local mode)
└── config-openai.toml       # Config for OpenAI mode
```

---

## 4. Project Structure

```
FluidVoice/
├── MVP/                            # Main application
│   ├── src/
│   │   ├── main.rs                 # Entry point, pipeline, audio, hotkey, typing
│   │   ├── config.rs               # AppConfig struct, TOML read/write
│   │   ├── telemetry.rs            # UsageStats, stats.json persistence
│   │   ├── tray.rs                 # System tray icon + context menu
│   │   ├── audio_feedback.rs       # Procedural beep sounds (rodio)
│   │   ├── model.rs                # Whisper model downloader (local feature)
│   │   └── transcriber.rs          # LocalTranscriber wrapper (local feature)
│   ├── dist/                       # Distribution files
│   ├── Cargo.toml                  # Dependencies + [features] config
│   └── README.md                   # Usage instructions
├── docs/
│   └── project/                    # This documentation
├── roadmap_fv.md                   # Active development roadmap
└── tasks_fv.md                     # Granular technical task checklist
```

---

## 5. Feature Flags

The app uses Cargo feature flags to compile in local or cloud mode:

```toml
[features]
default = ["local"]
local = ["dep:whisper-rs"]    # Local Whisper inference
openai = []                    # OpenAI API cloud transcription
```

> **Important:** `local` and `openai` are mutually exclusive. Build with only one at a time.

---

## 6. Key Development Workflows

### Adding a New Config Field

1. Add the field to `AppConfig` struct in `config.rs`.
2. Set a default in `impl Default for AppConfig`.
3. Access via `config.field_name` in `main.rs`.
4. Update `config.toml` template in `dist/`.

### Testing Audio Capture

```powershell
cd MVP
cargo run
# Press Ctrl+Shift+V to trigger recording
# Check recording.wav is generated
```

### Running Checks

```powershell
cd MVP

# Type checking
cargo check

# Lint
cargo clippy

# Both builds
cargo check --features local
cargo check --no-default-features --features openai
```

---

## 7. Coding Standards

### Rust

- Follow standard Rust conventions (`rustfmt`, `clippy`).
- Use `anyhow::Result` for error propagation.
- Avoid `unwrap()` in production paths — use `?` or `.context()`.
- Feature-gated code uses `#[cfg(feature = "local")]` / `#[cfg(feature = "openai")]`.

### General

- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/).
- All sensitive data (API keys, `.env`) is gitignored.

---

## 8. Debugging Tips

### Console Output (Debug Mode)

In debug mode, the console window is visible and all `println!` output appears. In release mode, the console is hidden (`windows_subsystem = "windows"`).

```powershell
# Debug mode (console visible)
cargo run

# Release mode (no console — use log files after Phase 4)
cargo run --release
```

### Common Issues

| Issue | Solution |
|-------|---------|
| `whisper-rs` build fails | Ensure MSVC Build Tools + CMake are installed |
| No audio capture | Check Windows Settings → Privacy → Microphone |
| Hotkey not working | May conflict with other apps using same shortcut |
| Model download fails | Check internet connection; manually download from Hugging Face |
| `enigo` typing fails | Some elevated/UWP apps reject simulated keystrokes |

---

## 9. Useful Links

| Resource | URL |
|----------|-----|
| whisper.cpp | https://github.com/ggerganov/whisper.cpp |
| whisper-rs | https://github.com/tazz4843/whisper-rs |
| cpal (audio I/O) | https://github.com/RustAudio/cpal |
| enigo (keyboard sim) | https://github.com/enigo-rs/enigo |
| rdev (global hotkeys) | https://github.com/Narsil/rdev |
| Conventional Commits | https://www.conventionalcommits.org |
