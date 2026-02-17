# Apple Intelligence Provider

## Purpose

Bridges to Apple Intelligence APIs for LLM/dictation when the selected provider is "apple-intelligence". Used for dictation cleanup (and possibly other flows) when AppleIntelligenceService.isAvailable is true. Implementation may use private or system APIs; availability is macOS/device dependent.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Networking/AppleIntelligenceProvider.swift`
- Availability / service check may be in same file or a small AppleIntelligenceService type.

## Depends on

- `Foundation`; possibly system frameworks for Apple Intelligence (exact API is implementation-specific)
- SettingsStore: selectedProviderID == "apple-intelligence"; no API key stored for this provider

## Consumed by

- Dictation AI path: when selectedProviderID is apple-intelligence and AppleIntelligenceService.isAvailable, use this provider instead of OpenAI-compatible HTTP.
- DictationAIPostProcessingGate: for apple-intelligence, isConfigured() returns AppleIntelligenceService.isAvailable.

## Contract

### Availability

- `AppleIntelligenceService.isAvailable` (or equivalent): true only when the device and OS support Apple Intelligence and the user has enabled it. False on Intel or older macOS.

### Processing

- Same logical contract as AIProvider for dictation: systemPrompt + userText → cleaned text. May be synchronous or async; no API key; may use on-device model.

## Invariants

- Do not send API key or base URL; Apple Intelligence is local/system.
- If unavailable, settings UI should show message and DictationAIPostProcessingGate should return false for apple-intelligence.
