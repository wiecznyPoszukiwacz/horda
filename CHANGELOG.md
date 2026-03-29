# Changelog

## [0.3.2] - 2026-03-29

### Fixed
- Launched apps no longer crash when horda is restarted — child processes are now detached into their own process group (`process_group(0)`)

## [0.3.1] - 2026-03-28

### Added
- Support for command arguments via `action_args` field in config (e.g. `action_args = ["--flag", "value"]`)

## [0.3.0] - 2026-03-22

### Added
- Verbose mode (`-v` flag) — logs config path, loaded commands, and search results on startup

## [0.2.1] - 2026-03-15

### Fixed
- Ignore Alt+Space when Ctrl, Shift or Meta is held

## [0.2.0]

### Added
- Initial release with global hotkey (Alt+Space), command launcher, keystroke sending
