# Antares Status — Evidence Required

> This file is deliberately conservative. The presence of a design specification or scaffold is not proof that a feature works.

## Snapshot
- Status: **AUDIT REQUIRED / NOT VERIFIED**
- Product: Antares IDE on Lapce foundation
- Active roadmap: [`ROADMAP.md`](ROADMAP.md)
- Product requirements: [`PRODUCT-SPEC.md`](PRODUCT-SPEC.md)
- Design rules: [`DESIGN-SYSTEM.md`](DESIGN-SYSTEM.md)
- Last status update: 2026-11-17 (repository reorganization only; no runtime feature verification performed)

## Evidence ledger
| Area | Status | Evidence required |
|---|---|---|
| Build/toolchain | NOT VERIFIED | exact toolchain + command logs |
| Full workspace tests | NOT VERIFIED | `cargo test --workspace` result |
| Antares agent runtime | NOT VERIFIED | process lifecycle + ACP E2E test |
| Agent UI | NOT VERIFIED | desktop smoke test + real event wiring |
| Multi-agent workflows | NOT VERIFIED | parallel/sequence/recovery tests |
| Generic MCP client | NOT VERIFIED | protocol contract tests |
| Plugin/skill manager | NOT VERIFIED | install/trust/rollback tests |
| Local API | NOT VERIFIED | auth/scope/loopback tests |
| Git UX | NOT VERIFIED | clone/push/credential tests |
| Discord RPC | NOT VERIFIED | opt-in/privacy/failure tests |
| Rebrand migration | NOT VERIFIED | upgrade/rollback OS matrix |
| Dead-code cleanup | NOT VERIFIED | reviewed inventory + before/after metrics |

## Rules for status updates
- Use `DONE` only with exact commit SHA and test/CI evidence.
- Use `IN PROGRESS`, `BLOCKED`, `NOT STARTED`, or `NOT VERIFIED` honestly.
- Record OS/toolchain, commands, exit codes, CI run URL, screenshots for UI work.
- Do not copy claims from old status documents without reproducing them.
