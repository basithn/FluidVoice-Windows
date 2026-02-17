# Simple Updater

## Purpose

Checks GitHub releases for a newer version than the running app; downloads the new .app (or package); replaces the current app and relaunches. Used for both manual "Check for Updates" and automatic update checks (triggered from AppDelegate).

## Source files

- `FluidVoice-1.5.5/Sources/Fluid/Services/SimpleUpdater.swift`
- Depends on: `AppUpdater` package (or equivalent GitHub API), `PromiseKit` for async flow

## Depends on

- `AppKit`, `Foundation`
- GitHub API (releases list, asset download URL)
- `Bundle.main.infoDictionary` for CFBundleShortVersionString
- `NSWorkspace.shared.openApplication(at:configuration:completionHandler:)` for relaunch
- File manager for copy/replace; possibly `Process` for relaunch helper

## Consumed by

- AppDelegate: `checkForUpdatesManually()` (SimpleUpdater.shared.checkAndUpdate), `checkForUpdatesAutomatically()` (SimpleUpdater.shared.checkForUpdate). SettingsStore: last check date, snooze version, auto-check enabled.

## Contract

### Check only

- `checkForUpdate(owner:repo:) async throws -> (hasUpdate: Bool, latestVersion: String)` — compare latest release tag with current version; return whether update exists and the tag. Does not download. Used by automatic check; SettingsStore updates last check date and may show notification.

### Check and install

- `checkAndUpdate(owner:repo:) async throws` — if update available, download asset (e.g. .zip or .app); replace running app in /Applications (or current location); relaunch; exit. On cancel (no update) throws PMKError.cancelled. Used by manual "Check for Updates".

### Version comparison

- Must handle "v1.2.3" vs "1.2.3"; 2-part vs 3-part versions (tolerant comparison as in codebase).

## Invariants

- Repo: `altic-dev/Fluid-oss`. Owner/repo are passed as parameters but currently fixed in call sites.
- Replace and relaunch must succeed atomically; if replace fails, do not quit app.
- Run on main thread for UI; network and file I/O can be background.

## Edge cases

- No network: check throws or returns hasUpdate: false; automatic check should not alert user (log only).
- Download incomplete: do not replace app; show error in manual flow.
- User has app elsewhere than /Applications: replace in place or prompt; implementation may copy to Applications then relaunch from there.
