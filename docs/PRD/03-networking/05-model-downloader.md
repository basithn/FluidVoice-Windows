# Model Downloader

## Purpose

Downloads ASR model assets (e.g. Whisper GGML/GGUF, Parakeet) from Hugging Face (or configured URL); caches to disk. For FluidAudio/Parakeet on arm64, also loads CoreML (MLModel) for encoder/decoder/joint/preprocessor. Used by WhisperProvider and FluidAudioProvider during prepare().

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Networking/ModelDownloader.swift`

## Depends on

- `Foundation`, `CoreML` (for Parakeet); `#if arch(arm64)` FluidAudio
- URLSession for download; FileManager for cache paths
- Hugging Face repo IDs and file names (e.g. FluidInference/FluidAudio, whisper model filenames)

## Consumed by

- WhisperProvider (download Whisper model to cache; path passed to SwiftWhisper).
- FluidAudioProvider / FluidAudio (Parakeet v2/v3: download + load MLModel instances).

## Contract

### Download

- Download file from URL to cache directory; report progress 0.0–1.0 via callback; return local URL on success.
- Cache location: application support or caches directory; subdir by model/repo.
- If file already present (and optionally valid), may skip download.

### Parakeet (arm64 only)

- Load encoder, decoder, joint, optional preprocessor/mel encoder from downloaded .mlmodelc or similar; use MLModelConfiguration for compute units; return or store in FluidAudio manager.
- Model variants: v2 vs v3; file names and repo from FluidAudio package or documented in code.

### Whisper

- Model size maps to filename (e.g. ggml-base.bin, ggml-small.en.bin); repo and path pattern in implementation.
- After download, WhisperProvider initializes SwiftWhisper with local path.

## Invariants

- Do not block main thread; progress and completion callbacks may be called on background or main as documented in implementation.
- Concurrent downloads for same key should be deduplicated (e.g. ModelDownloadRegistry in ASRService or inside downloader).

## Edge cases

- Network failure: throw or call progress with error; caller (provider prepare) may retry or show error.
- Disk full: handle write failure; clear cache on user request via provider clearCache().
