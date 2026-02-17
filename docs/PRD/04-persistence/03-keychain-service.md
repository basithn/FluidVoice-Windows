# Keychain Service

## Purpose

Stores and retrieves provider API keys in the macOS Keychain as a single generic-password item (e.g. one JSON or plist keyed by provider ID). Used only for secrets; not for general app storage.

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Persistence/KeychainService.swift`

## Depends on

- `Foundation`, `Security`
- SecItemAdd, SecItemCopyMatching, SecItemDelete, SecItemUpdate; kSecClassGenericPassword, kSecAttrService, kSecAttrAccount

## Consumed by

- SettingsStore: getAPIKey, setAPIKey, and migration from legacy ProviderAPIKeys; AISettings may probe Keychain access.

## Contract

### Constants

- Service: `com.fluidvoice.provider-api-keys` (or equivalent in code).
- Account: `fluidApiKeys` (or single account for the blob).

### API

- `storeKey(_ key: String, for providerID: String) throws` — merge providerID → key into stored dict; save to Keychain.
- `fetchKey(for providerID: String) throws -> String?`
- `deleteKey(for providerID: String) throws`
- `containsKey(for providerID: String) -> Bool`
- `allProviderIDs() throws -> [String]`
- `fetchAllKeys() throws -> [String: String]`
- `storeAllKeys(_ values: [String: String]) throws`
- `legacyProviderEntries() throws -> [String: String]` — for migration from old Keychain format; removeLegacyEntries after merge.

### Storage shape

- Stored value is a single item; value is encoded dict [providerID: apiKey]. Read: decode to dict; write: encode dict and SecItemAdd/Update.

### Errors

- KeychainServiceError: invalidData, unhandled(OSStatus). Use SecCopyErrorMessageString for user-facing message when possible.

## Invariants

- Keys are trimmed (whitespace) before store; empty string is valid (e.g. clear key for a provider).
- Do not log or persist raw keys outside Keychain; SettingsStore should not keep keys in memory longer than needed for a request.

## Edge cases

- Keychain locked (e.g. after sleep): SecItemCopyMatching may fail; caller should handle and optionally prompt user.
- Multiple keychain items with same service/account: legacy migration may return multiple; merge then remove legacy.
