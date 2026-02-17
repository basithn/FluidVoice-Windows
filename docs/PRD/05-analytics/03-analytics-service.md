# Analytics Service

## Purpose

Initializes PostHog (or equivalent) and provides a single capture API for typed events and properties. Respects user consent (ShareAnonymousAnalytics); may no-op when consent is off.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Analytics/AnalyticsService.swift`
- `FluidVoice-1.5.5/Sources/Fluid/Analytics/AnalyticsConfig.swift`
- `FluidVoice-1.5.5/Sources/Fluid/Analytics/AnalyticsIdentityStore.swift`

## Depends on

- PostHog (posthog-ios); AnalyticsConfig (API key, host)
- SettingsStore.shared.shareAnonymousAnalytics
- AnalyticsEvent, AnalyticsBuckets (for bucketing)
- CPUArchitecture (optional: send arch in properties)

## Consumed by

- AppDelegate (bootstrap, app_open, app_first_open); ASRService or delivery path (transcription_completed, output_delivered); CommandModeService, RewriteModeService; error handlers; SettingsView (consent changed); PostTranscriptionEditTracker.

## Contract

### Bootstrap

- `AnalyticsService.shared.bootstrap()` — set up PostHog with key and host from AnalyticsConfig; identify or set anonymous ID if needed. Call once at launch (AppDelegate).

### Capture

- `func capture(_ event: AnalyticsEvent, properties: [String: Any]? = nil)` — if ShareAnonymousAnalytics is false, return without sending. Otherwise send event name (event.rawValue) and properties to PostHog. Properties should be JSON-serializable and low-cardinality.

### Identity

- AnalyticsIdentityStore.ensureFirstOpenRecorded() — if first run, set flag (UserDefaults), return true so caller can emit app_first_open. Subsequent calls return false.
- Anonymous ID: generated or read from storage; used for PostHog distinct_id.

## Invariants

- Do not send before bootstrap. Do not send when consent is off.
- Run on main thread for consistency with SettingsStore reads; PostHog may queue on its own thread.
