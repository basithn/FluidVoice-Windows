# Analytics Events and Schema

## Purpose

Defines the set of analytics events and their payload shapes so agents and code can emit consistent, low-cardinality events. Used by AnalyticsService.capture(...).

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Analytics/AnalyticsEvent.swift`
- `FluidVoice-1.5.5/Sources/Fluid/Analytics/AnalyticsBuckets.swift`

## Event enum (AnalyticsEvent)

| Event | Raw value | When to send |
|-------|-----------|--------------|
| appFirstOpen | app_first_open | First launch (AnalyticsIdentityStore.ensureFirstOpenRecorded). |
| appOpen | app_open | Every launch; properties: accessibility_trusted (Bool). |
| analyticsConsentChanged | analytics_consent_changed | When user toggles ShareAnonymousAnalytics. |
| transcriptionCompleted | transcription_completed | After final transcription and optional AI; include mode, length, model bucket. |
| outputDelivered | output_delivered | After text was typed or copied; properties: mode, output_method. |
| postTranscriptionEdit | post_transcription_edit | When user edits text after delivery (PostTranscriptionEditTracker). |
| commandModeRunCompleted | command_mode_run_completed | After command-mode run (success or failure). |
| rewriteRunCompleted | rewrite_run_completed | After rewrite/write flow completed. |
| meetingTranscriptionCompleted | meeting_transcription_completed | After meeting transcription session. |
| customPromptUsed | custom_prompt_used | When a custom dictation prompt profile is used. |
| errorOccurred | error_occurred | On handled errors; properties: domain, message (bucketed). |

## Supporting enums

- **AnalyticsMode:** dictation | command | rewrite | meeting.
- **AnalyticsOutputMethod:** typed | clipboard | history_only.
- **AnalyticsErrorDomain:** asr | llm | typing | hotkey | update | other.

## Properties (typical)

- Keep cardinality low: bucket model names (AnalyticsBuckets), truncate messages, use enums not free-form strings where possible.
- Do not send PII or full transcript text; use counts, booleans, and bucket IDs.
- accessibility_trusted: Bool (for app_open).

## Invariants

- All capture calls go through AnalyticsService; respect ShareAnonymousAnalytics (no send when false).
- Event names must match server/schema if PostHog has a schema; use AnalyticsEvent.rawValue.
