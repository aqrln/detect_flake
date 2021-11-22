# detect_flake

Run many instances of the same command in parallel to find abnormal behavior or check if a test is flaky.

## Installation

```sh
cargo install detect_flake
```

## Usage

```
detect_flake [FLAGS] [OPTIONS] --command <command>
```

Flags:

- `-h, --help` — Prints help information
- `-V, --version` — Prints version information
- `-i, --inherit-stdio` — Inherit stdio instead of redirecting to `/dev/null`

Options:

- `-c, --command <command>` — Command to run
- `-r, --runs-per-thread <runs-per-thread>` — Number of serial runs per each thread [default: 1]
- `-t, --threads <threads>` — Number of parallel threads [default: 100]
