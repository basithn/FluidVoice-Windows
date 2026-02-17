# Analytics Index

Analytics covers product events, identity, and PostHog integration. All under `FluidVoice-1.5.5/Sources/Fluid/Analytics/`.

| Component | File | Purpose |
|-----------|------|--------|
| **AnalyticsEvent** | AnalyticsEvent.swift | Typed event names (app_first_open, transcription_completed, etc.) and supporting enums. |
| **AnalyticsConfig** | AnalyticsConfig.swift | PostHog API key, host; feature flags or config constants. |
| **AnalyticsIdentityStore** | AnalyticsIdentityStore.swift | First-open tracking; anonymous ID; ensureFirstOpenRecorded(). |
| **AnalyticsService** | AnalyticsService.swift | Bootstrap (PostHog init); capture(event, properties); respects ShareAnonymousAnalytics. |
| **AnalyticsBuckets** | AnalyticsBuckets.swift | Bucketing helpers for low-cardinality reporting. |
| **PostTranscriptionEditTracker** | PostTranscriptionEditTracker.swift | Tracks edits after transcription; may emit post_transcription_edit event. |

## Data flow

- AppDelegate: bootstrap AnalyticsService; on first open capture app_first_open; on every launch capture app_open (with accessibility_trusted).
- ASR/delivery: capture transcription_completed, output_delivered (mode, output_method).
- Command/Rewrite: capture command_mode_run_completed, rewrite_run_completed.
- Errors: capture error_occurred with domain and message (bucketed).
- Consent: ShareAnonymousAnalytics toggles whether any events are sent.

## Dependencies

- Analytics depends on SettingsStore (consent). No UI dependency.
- PostHog SDK (posthog-ios) for send; AnalyticsConfig for key/host.
