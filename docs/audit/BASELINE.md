# Baseline Audit Ledger

- Commit SHA: `930826b377fbc00297a71a5ba2858d4e972e8c4b`
- Branch: `main`
- OS/toolchain: Ubuntu 22.04 LTS (GitHub Actions runner), Rust `1.98.0` (Edition 2024), Cargo `1.98.0`
- Date: 2026-11-17

## Commands and evidence
| Check | Command | Result | Log/artifact |
|---|---|---|---|
| Scaffold | `bash scripts/scaffold-check.sh` | PASS | CI Run [44080240671](https://github.com/dhapidpad/Antares-Ide/actions/runs/44080240671) |
| Whitespace | `git diff --check` | PASS | CI Run [44080240671](https://github.com/dhapidpad/Antares-Ide/actions/runs/44080240671) |
| Format (agent) | `cargo fmt -p antares-agent -- --check` | PASS | CI Run [44080240671](https://github.com/dhapidpad/Antares-Ide/actions/runs/44080240671) |
| Check (agent) | `cargo check -p antares-agent` | PASS | CI Run [44080240671](https://github.com/dhapidpad/Antares-Ide/actions/runs/44080240671) |
| Tests (agent) | `cargo test -p antares-agent` (61 tests) | PASS | CI Run [44080240671](https://github.com/dhapidpad/Antares-Ide/actions/runs/44080240671) |
| Clippy (agent) | `cargo clippy -p antares-agent --all-targets -- -D warnings` | PASS | CI Run [44080240671](https://github.com/dhapidpad/Antares-Ide/actions/runs/44080240671) |
| Commit SHA | `5790bad49510a5aa6795234c0383c45ac7521bf0` |
| Check (workspace) | `cargo check --workspace` | PASS (3m27s) | CI Run [44082026237](https://github.com/dhapidpad/Antares-Ide/actions/runs/44082026237) |
| Tests (workspace) | `cargo test --workspace` | PASS (90+ tests, 0 failed, 1m47s) | CI Run [44082026237](https://github.com/dhapidpad/Antares-Ide/actions/runs/44082026237) |
| Clippy (workspace) | `cargo clippy --workspace --all-targets -- -D warnings` | NOT RUN | — |
| Dependencies | `cargo tree -d` | NOT RUN | — |

## Performance baseline
- Agent CI Fast Check Duration: 24–25 seconds.
- Full Workspace Check Duration (Linux with graphics deps): 3 minutes 27 seconds.
- Runtime idle RSS/CPU, peak RSS, binary size: NOT MEASURED (desktop native build not yet executed).

## UI evidence
Dark/Light screenshots, narrow layout and keyboard smoke test: NOT CAPTURED (pending desktop build).
