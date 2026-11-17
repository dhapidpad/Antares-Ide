# Baseline Audit Template

- Commit SHA: NOT RECORDED
- Branch: NOT RECORDED
- OS/toolchain: NOT RECORDED
- Date: 2026-11-17

## Commands and evidence
| Check | Command | Result | Log/artifact |
|---|---|---|---|
| Metadata | `cargo metadata --format-version 1 --no-deps` | NOT RUN | — |
| Format | `cargo fmt --check` | NOT RUN | — |
| Check | `cargo check --workspace --all-targets` | NOT RUN | — |
| Tests | `cargo test --workspace` | NOT RUN | — |
| Clippy | `cargo clippy --workspace --all-targets -- -D warnings` | NOT RUN | — |
| Dependencies | `cargo tree -d` | NOT RUN | — |

## Performance baseline
Startup, idle RSS/CPU, peak RSS, binary size and build time: NOT MEASURED.

## UI evidence
Dark/Light screenshots, narrow layout and keyboard smoke test: NOT CAPTURED.

Do not replace NOT RUN/NOT MEASURED until the command or measurement actually exists.
