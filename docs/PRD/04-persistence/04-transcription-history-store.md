# Transcription History Store

## Purpose

Stores a bounded list of completed transcription entries (raw text, processed text, app name, window title, timestamp) for the History view. Supports add, load, delete, and selection. Persisted in UserDefaults.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Persistence/TranscriptionHistoryStore.swift`

## Depends on

- `Combine`, `Foundation`
- UserDefaults; Keys.transcriptionHistory

## Consumed by

- ASRService or delivery path: after typing or copying, call addEntry(...) if SaveTranscriptionHistory is enabled.
- TranscriptionHistoryView: displays entries, selectedEntryID; copy/type-from-history actions.
- ContentView: may navigate to history and pass store to view.

## Contract

### Model

- **TranscriptionHistoryEntry:** id (UUID), timestamp (Date), rawText, processedText, appName, windowTitle, characterCount (derived), wasAIProcessed (derived). Codable, Identifiable, Equatable.
- previewText: first ~80 chars for list row.
- relativeTimeString, fullDateString: formatted for display.

### Store API

- `static let shared`; `@MainActor`; `ObservableObject`.
- `@Published private(set) var entries: [TranscriptionHistoryEntry]`
- `@Published var selectedEntryID: UUID?`
- `var selectedEntry: TranscriptionHistoryEntry?` — entry for selectedEntryID.
- `func addEntry(rawText:processedText:appName:windowTitle:)` — append; skip if processedText empty; trim to max count (e.g. 100); persist.
- `func loadEntries()` — read from UserDefaults; decode; set entries.
- `func deleteEntry(id:)` — remove from array; persist; clear selectedEntryID if needed.
- `func clearAll()` — remove all; persist.

### Persistence

- Key: `TranscriptionHistoryEntries` (or as in Keys). Value: JSON-encoded array of entries.
- Max entries: implementation-defined (e.g. 100); drop oldest when over.

## Invariants

- Add only when SaveTranscriptionHistory (SettingsStore) is true; otherwise skip addEntry.
- UI must not hold large text in memory unnecessarily; list shows previewText; full text on selection.
