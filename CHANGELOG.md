<!-- next-header -->

## Unreleased

Commits: <https://github.com/aqrln/detect_flake/compare/v0.6.0...HEAD>

## Version 0.6.0 (2024-01-23)

Commits: <https://github.com/aqrln/detect_flake/compare/v0.5.7...v0.6.0>

### Added

- Proper parser for command arguments which mimics the POSIX shell. You can now
  use commands with arguments that contain spaces by wrapping them in quotes:

  ```
  detect_flake -c 'echo "hello world"'
  ```

  Conversely, if you relied on quotes used in the commands verbatim, they may
  now need to be escaped using backslashes.

- Retroactively added the changelog since the past few versions which will now
  be maintained for all future releases.

## Version 0.5.7 (2024-01-21)

Commits: <https://github.com/aqrln/detect_flake/compare/v0.5.6...v0.5.7>

No code changes in this release, just a fixup in README for the published packages.

## Version 0.5.6 (2024-01-21)

Commits: <https://github.com/aqrln/detect_flake/compare/v0.5.5...v0.5.6>

### Added

- Publish pre-built binaries for ARM64
- Installation instructions in README

## Version 0.5.5 (2023-11-25)

Commits: <https://github.com/aqrln/detect_flake/compare/v0.5.4...v0.5.5>

### Changed

- Dual license under MIT and Apache-2.0

## Version 0.5.4 (2023-11-25)

Commits: <https://github.com/aqrln/detect_flake/compare/v0.5.3...v0.5.4>

### Added

- Pre-built binaries and installers
- Homebrew package
- npm package
- Nix flake

### Changed

- Improve the error message when reporting unsuccessful runs.
