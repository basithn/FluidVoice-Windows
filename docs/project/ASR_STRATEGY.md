# ASR Strategy for Windows

## 1. Problem Statement

FluidVoice macOS relies on three speech recognition engines:

1. **Parakeet TDT v3/v2** (via FluidAudio) — Apple Silicon CoreML, fastest, best quality.
2. **Apple Speech** (`SFSpeechRecognizer`) — system-level, low latency, English + other languages.
3. **Whisper** (via SwiftWhisper) — cross-platform fallback, uses whisper.cpp under the hood.

**None of the first two are available on Windows.** This document defines our ASR strategy to deliver equivalent quality and performance.

---

## 2. Chosen Strategy

### Primary Engine: **Whisper via whisper.cpp**

Whisper is the only engine from the macOS version that works cross-platform. We will use `whisper-rs` (Rust bindings to whisper.cpp) as the primary and default ASR engine.

| Aspect | Detail |
|--------|--------|
| **Library** | `whisper-rs` (wraps `whisper.cpp`) |
| **Models** | OpenAI Whisper GGML format: `tiny`, `base`, `small`, `medium`, `large-v3` |
| **Default model** | `base.en` (good accuracy-to-speed ratio for English) |
| **Inference** | CPU by default; GPU via CUDA or Vulkan backends (opt-in) |
| **Languages** | Multilingual models (`base`, `small`, etc.) support 99 languages; `.en` variants are English-optimized |
| **Latency target** | < 3s for a 10s clip on a modern CPU (base model) |

### Optional Engines (Future)

| Engine | When to consider | Pros | Cons |
|--------|------------------|------|------|
| **Windows Speech Recognition** | Users wanting near-zero latency, basic dictation | Built-in, no download, streaming | Lower accuracy, limited customization |
| **Azure Cognitive Services Speech** | Users needing cloud accuracy, multilingual | High accuracy, real-time streaming, 100+ languages | Requires API key, internet, cost |
| **faster-whisper** (Python) | If we need CTranslate2 optimizations | 4× faster than whisper.cpp on some hardware | Python dependency, subprocess overhead |
| **Whisper.net** | If we ever add C# components | .NET-native Whisper | Only relevant if stack changes |

---

## 3. Model Management

### 3.1 Model Distribution

Models are **not bundled** with the installer (too large). Instead:

1. On **first run**, the app detects no model is present.
2. A setup wizard prompts the user to select a model size.
3. The model is downloaded from Hugging Face (or a CDN mirror).
4. Models are stored in `%APPDATA%\FluidVoice\models\`.

### 3.2 Model Sizes

| Model | Params | Disk Size | RAM Usage | Relative Speed | Relative Accuracy |
|-------|--------|-----------|-----------|----------------|-------------------|
| `tiny.en` | 39M | ~75 MB | ~390 MB | ★★★★★ | ★★☆☆☆ |
| `base.en` | 74M | ~142 MB | ~500 MB | ★★★★☆ | ★★★☆☆ |
| `small.en` | 244M | ~466 MB | ~1 GB | ★★★☆☆ | ★★★★☆ |
| `medium.en` | 769M | ~1.5 GB | ~2.6 GB | ★★☆☆☆ | ★★★★★ |
| `large-v3` | 1550M | ~3.1 GB | ~4.7 GB | ★☆☆☆☆ | ★★★★★ |

**Recommendation**: Default to `base.en` and let users upgrade in Settings.

### 3.3 GPU Acceleration

whisper.cpp supports multiple backends:

| Backend | Requirement | Speedup |
|---------|-------------|---------|
| **CPU (default)** | None | Baseline |
| **CUDA** | NVIDIA GPU + CUDA toolkit | 5–15× |
| **Vulkan** | Any modern GPU (AMD, NVIDIA, Intel) | 3–8× |
| **OpenCL** | Any OpenCL-capable GPU | 2–5× |

Strategy:
1. Ship CPU-only by default (zero setup).
2. Detect GPU availability at startup.
3. Offer one-click GPU acceleration enable in Settings (may require downloading a GPU-optimized binary).

---

## 4. Audio Pipeline

```
Microphone
    │
    ▼
WASAPI Capture (via cpal)
    │  16-bit PCM, 16 kHz, mono
    ▼
Ring Buffer (in memory)
    │
    │  [hotkey released → flush]
    ▼
whisper-rs inference (dedicated thread)
    │
    ▼
Transcribed text (UTF-8 string)
    │
    ▼
[Optional: AI post-processing]
    │
    ▼
Typing service → focused app
```

### Audio Format Requirements

- **Sample rate**: 16,000 Hz (Whisper's native rate)
- **Channels**: Mono
- **Bit depth**: 16-bit signed integer (i16) or 32-bit float (f32, converted)
- **Resampling**: If mic doesn't support 16 kHz natively, resample in software (`rubato` crate or `cpal` built-in)

---

## 5. Performance Benchmarks (Target)

| Scenario | Model | Hardware | Audio Length | Target Latency |
|----------|-------|----------|-------------|----------------|
| Quick command | `base.en` | CPU, i7/Ryzen 7 | 2–3 s | < 1.5 s |
| Normal dictation | `base.en` | CPU, i7/Ryzen 7 | 10 s | < 3 s |
| Long dictation | `base.en` | CPU, i7/Ryzen 7 | 30 s | < 8 s |
| Normal dictation | `base.en` | GPU (CUDA) | 10 s | < 1 s |
| Normal dictation | `small.en` | CPU, i7/Ryzen 7 | 10 s | < 6 s |

These targets should be validated during Phase 1 development.

---

## 6. Replacing Parakeet Quality

Parakeet TDT v3 (on macOS) offers higher accuracy than Whisper `base` with similar speed, thanks to Apple Silicon optimization. To approach this quality on Windows:

1. **Use `small.en` or `medium.en`** for users who need higher accuracy and have the hardware.
2. **Enable GPU acceleration** — CUDA makes even `medium.en` fast.
3. **AI post-processing** — grammar/punctuation correction via LLM can compensate for Whisper's raw output quality.
4. **Fine-tuned models** — whisper.cpp supports custom fine-tuned models; explore domain-specific fine-tuning over time.

---

## 7. Open Questions

| # | Question | Impact | Decision Needed By |
|---|----------|--------|--------------------|
| 1 | Should we support streaming transcription (real-time partial results)? | UX improvement; significant implementation effort | Phase 2 |
| 2 | Should we bundle a tiny model in the installer for zero-download first use? | Better first-run experience; adds ~75 MB to installer | Phase 1 |
| 3 | Should we support Whisper via Python `faster-whisper` as an alternative engine? | Better perf on some hardware; adds Python dependency | Phase 3 |
| 4 | Should we support Azure Speech as a paid cloud option? | Useful for enterprise; requires billing/subscription | Phase 4 |
