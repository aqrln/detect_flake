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
- `-p, --print-failing-output` — Print the stdout and stderr of unsucessful runs only

Options:

- `-c, --command <command>` — Command to run
- `-r, --runs-per-thread <runs-per-thread>` — Number of serial runs per each thread [default: 100]
- `-t, --threads <threads>` — Number of parallel threads [default: 10]
