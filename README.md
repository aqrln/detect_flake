# detect_flake

Run many instances of the same command in parallel to find abnormal behavior or check if a test is flaky.

## Installation

See the [GitHub Releases](https://github.com/aqrln/detect_flake/releases) for installation scripts and pre-built binaries.

You can also use one of the following package managers.

### Install using cargo

```sh
cargo install detect_flake
```

### Install using npm

```sh
npm install detect_flake
```

### Install using Homebrew

```sh
brew install aqrln/homebrew-tap/detect_flake
```

### Install using Nix

```sh
nix profile install github:aqrln/detect_flake
```

(or run it right away without installing with `nix run github:aqrln/detect_flake -- <ARGUMENTS>`)

## Usage

```
detect_flake [FLAGS] [OPTIONS] --command <command>
```

Flags:

- `-h, --help` — Prints help information
- `-V, --version` — Prints version information
- `-i, --inherit-stdio` — Inherit stdio instead of redirecting to `/dev/null`
- `-p, --print-failing-output` — Print the stdout and stderr of unsuccessful runs only
- `-e, --exit-early-on-error` — Exit early the first time the command returns a non-zero error code

Options:

- `-c, --command <command>` — Command to run
- `-r, --runs-per-thread <runs-per-thread>` — Number of serial runs per each thread [default: 100]
- `-t, --threads <threads>` — Number of parallel threads [default: 10]
