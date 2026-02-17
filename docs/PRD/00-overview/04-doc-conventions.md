# PRD Document Conventions

Conventions used in this PRD so agents and humans can parse and use it consistently.

## Paths

- **Repository root:** The folder containing `FluidVoice-1.5.5` and `docs`.
- **Source root:** `FluidVoice-1.5.5/Sources/Fluid/`. All Swift source paths in the PRD are relative to the repo root and start with this prefix unless stated otherwise.
- **PRD root:** `docs/PRD/`. Links between PRD docs are relative to this (e.g. `../02-services/01-services-index.md`).

## Identifiers

- **Swift types:** Full type name as in code (e.g. `ASRService`, `SettingsStore.SpeechModel`).
- **UserDefaults keys:** Exact key string from `SettingsStore.Keys` or equivalent (e.g. `HotkeyShortcutKey`, `SelectedSpeechModel`).
- **Keychain:** Service/account names as in `KeychainService` (e.g. `com.fluidvoice.provider-api-keys`).
- **Provider IDs:** Lowercase, as in `ModelRepository.builtInProviderIDs` (e.g. `openai`, `ollama`, `apple-intelligence`).

## Sections in Module Docs

Each module or component doc should include where applicable:

- **Purpose:** One or two sentences on what the component does.
- **Source files:** List of paths (relative to repo root) that implement this component.
- **Depends on:** Other modules or types this component uses (for layering and impact analysis).
- **Consumed by:** Callers (modules/views) that use this component.
- **Contracts:** Types, method signatures, key names (enough to implement or mock).
- **Invariants / edge cases:** Non-obvious guarantees or failure modes.
- **Threading:** Main vs background; actors if any.

## Code Snippets

- Snippets are illustrative; when in doubt, the source file is the authority.
- Swift signatures may be simplified (e.g. default args omitted) but must match the intended contract.

## Versioning

- PRD reflects the codebase as of the last update (align with `FluidVoice-1.5.5` tag or current main). No separate version field; doc date or repo ref can be used for staleness checks.
