# Transcription Providers

## Purpose

`TranscriptionProvider` is the protocol for all ASR backends. Implementations: FluidAudio (Parakeet), Apple Speech, Apple Speech Analyzer, Whisper. ASRService selects one based on `SettingsStore.selectedSpeechModel` and uses it for `transcribe(_ samples: [Float])` and prepare/clearCache.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Services/TranscriptionProvider.swift` — protocol and `ASRTranscriptionResult`
- `FluidVoice-1.5.5/Sources/Fluid/Services/FluidAudioProvider.swift`
- `FluidVoice-1.5.5/Sources/Fluid/Services/AppleSpeechProvider.swift`
- `FluidVoice-1.5.5/Sources/Fluid/Services/AppleSpeechAnalyzerProvider.swift`
- `FluidVoice-1.5.5/Sources/Fluid/Services/WhisperProvider.swift`

## Protocol contract

```text
protocol TranscriptionProvider {
  var name: String { get }
  var isAvailable: Bool { get }
  var isReady: Bool { get }
  func prepare(progressHandler: ((Double) -> Void)?) async throws
  func transcribe(_ samples: [Float]) async throws -> ASRTranscriptionResult
  func modelsExistOnDisk() -> Bool   // default: false
  func clearCache() async throws     // default: no-op
}

struct ASRTranscriptionResult {
  let text: String
  let confidence: Float  // default 1.0
}
```

- **samples:** 16 kHz mono Float PCM.
- **prepare:** Download or load model; progress 0.0–1.0. May throw if permission or network fails.
- **transcribe:** Synchronous-style async; must not be called concurrently (ASRService uses TranscriptionExecutor).

## Implementations

| Provider | Availability | Notes |
|---------|-------------|--------|
| **FluidAudioProvider** | `#if arch(arm64)` | Parakeet TDT v2/v3 via FluidAudio; CoreML; Apple Silicon only. Intel stub throws. |
| **AppleSpeechProvider** | Always (macOS) | SFSpeechRecognizer; on-device; no download. |
| **AppleSpeechAnalyzerProvider** | macOS 26+ | Speech Analyzer API; different API surface. |
| **WhisperProvider** | Always | SwiftWhisper; model size from settings (e.g. base, small); downloads from Hugging Face. |

## Architecture detection

- `TranscriptionProvider.swift` defines `CPUArchitecture` (`.applesilicon` / `.intel`) via `#if arch(arm64)` so UI can hide FluidAudio on Intel.

## Depends on

- FluidAudio: `FluidAudio` package.
- Apple: `AVFoundation`, `Speech`.
- Whisper: `SwiftWhisper`; `ModelDownloader` for download.

## Consumed by

- `ASRService` only; no other module implements or holds a provider directly.
