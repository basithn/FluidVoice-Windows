# ASR Service

## Purpose

Orchestrates the full dictation ASR pipeline: microphone capture, chunked audio buffering, model selection (FluidAudio / Apple Speech / Whisper), download/prepare, transcription execution (serialized via actor), and publishing of partial/final text and status. Used only for the main dictation flow (not meeting transcription).

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Services/ASRService.swift`
- Lazy ownership: `FluidVoice-1.5.5/Sources/Fluid/Services/AppServices.swift` (creates and holds `ASRService`)

## Depends on

- `Accelerate`, `AVFoundation`, `Combine`, `Foundation`, `AppKit`, `AudioToolbox`, `CoreAudio`
- `#if arch(arm64)`: `FluidAudio`
- `TranscriptionProvider` (FluidAudioProvider, AppleSpeechProvider, AppleSpeechAnalyzerProvider, WhisperProvider)
- `SettingsStore`, `DebugLogger`, `ModelDownloader` (via providers), `TranscriptionSoundPlayer`, `MediaPlaybackService` (optional pause), `ThreadSafeAudioBuffer`, `AudioStartupGate`

## Consumed by

- `AppServices` (exposes `asr`); ContentView and UI use `appServices.asr`
- `MenuBarManager` (configured with ASR for Start/Stop)
- `GlobalHotkeyManager` (start/stop recording callbacks)
- `NotchOverlayManager` (audio level publisher for visualization)

## Contract

### Type and threading

- `@MainActor final class ASRService: ObservableObject`

### Published state

- `isRunning: Bool` — recording active
- `finalText: String` — last finalized transcription
- `partialTranscription: String` — streaming partial
- `micStatus: AVAuthorizationStatus`
- `isAsrReady: Bool` — model ready
- `isDownloadingModel: Bool`, `isLoadingModel: Bool`, `modelsExistOnDisk: Bool`
- `downloadProgress: Double?`, `downloadingModelId: String?`
- `errorTitle`, `errorMessage`, `showError` — error alert

### Key methods

- `initialize()` — call once after UI ready; checks mic permission, sets up audio pipeline, does not start recording.
- `startRecording() async` — start capture and transcription loop.
- `stopAndProcess() async -> String` — stop capture, run final transcription, return final text; may apply filler-word removal and GAAV; optionally run AI post-processing and return processed text.
- `cancelRecording()` — stop without processing.
- `checkIfModelsExist()` async — updates `modelsExistOnDisk` for current provider.
- `prepareModel(progressHandler:)` async — download/load model for current `selectedSpeechModel`; uses `TranscriptionExecutor` to serialize with other transcription work.

### Provider selection

- `transcriptionProvider` getter: derived from `SettingsStore.shared.selectedSpeechModel` (FluidAudio / Apple Speech / Apple Speech Analyzer / Whisper; Whisper size from same setting). On Intel, FluidAudio is unavailable → fallback to Whisper.

### Internal actors

- `TranscriptionExecutor`: serializes all `transcribe(_:)` and prepare work to avoid concurrent CoreML/load.
- `ModelDownloadRegistry`: deduplicates concurrent downloads by key.

### Invariants

- Do not call `startRecording` or heavy prepare before `AudioStartupGate` / UI ready; ContentView delays ASR init and `initialize()` by 1.5s after `signalUIReady()`.
- Audio format: 16 kHz mono Float samples; chunk duration ~0.6s for transcription loop.
- After stop, AI post-processing (if enabled) runs on main actor via `DictationAIPostProcessingGate` and selected provider/model; result is what gets returned and optionally typed.

## Edge cases

- Mic permission denied: `micStatus` reflects it; recording fails gracefully.
- Model download in progress: `isDownloadingModel` true; start can wait or show status.
- First transcription may be slower (model warm-up); `hasCompletedFirstTranscription` used for UX.
- Escape key during recording: handled in ContentView/NotchOverlayManager; calls `cancelRecording()`.
