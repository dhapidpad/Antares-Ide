# Baseline Audit Ledger

- Commit SHA: `de16119a8789f6b9ede47558db6bc1a037a8b407`
- Branch: `main`
- OS/toolchain: Ubuntu 22.04/24.04 LTS (GitHub Actions runner), Rust `1.98.0` (Edition 2024), Cargo `1.98.0`
- Date: 2026-11-17

## Commands and evidence
| Check | Command | Result | Log/artifact |
|---|---|---|---|
| Scaffold | `bash scripts/scaffold-check.sh` | PASS | CI Run [44094917730](https://github.com/dhapidpad/Antares-Ide/actions/runs/44094917730) |
| Whitespace | `git diff --check` | PASS | CI Run [44094917730](https://github.com/dhapidpad/Antares-Ide/actions/runs/44094917730) |
| Format (agent) | `cargo fmt -p antares-agent -- --check` | PASS | CI Run [44094917730](https://github.com/dhapidpad/Antares-Ide/actions/runs/44094917730) |
| Check (agent) | `cargo check -p antares-agent` | PASS | CI Run [44094917730](https://github.com/dhapidpad/Antares-Ide/actions/runs/44094917730) |
| Tests (agent) | `cargo test -p antares-agent` (61 tests) | PASS | CI Run [44094917730](https://github.com/dhapidpad/Antares-Ide/actions/runs/44094917730) |
| Clippy (agent) | `cargo clippy -p antares-agent --all-targets -- -D warnings` | PASS | CI Run [44094917730](https://github.com/dhapidpad/Antares-Ide/actions/runs/44094917730) |
| Check (workspace) | `cargo check --workspace` | PASS | CI Run [44095115155](https://github.com/dhapidpad/Antares-Ide/actions/runs/44095115155) |
| Tests (workspace) | `cargo test --workspace` | PASS (100+ tests, 0 failed, layout presets & icon registry covered) | CI Run [44095115155](https://github.com/dhapidpad/Antares-Ide/actions/runs/44095115155) |
| Clippy (workspace) | `cargo clippy --workspace --all-targets -- -D warnings` | NOT RUN | — |
| Dependencies | `cargo tree -d` | NOT RUN | — |

## Performance baseline
- Agent CI Fast Check Duration: 24–25 seconds.
- Full Workspace Check Duration (Linux with graphics deps): 3 minutes 27 seconds.
- Runtime idle RSS/CPU, peak RSS, binary size: NOT MEASURED (desktop native build not yet executed).

## UI evidence
Dark/Light screenshots, narrow layout and keyboard smoke test: NOT CAPTURED (pending desktop build).
