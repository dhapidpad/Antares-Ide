# Antares Status — Evidence Required

> This file is deliberately conservative. The presence of a design specification or scaffold is not proof that a feature works.

## Snapshot
- Status: **IN PROGRESS (PHASE 4 COMPLETED / PHASE 5 ACTIVE)**
- Product: Antares IDE on Lapce foundation
- Active roadmap: [`ROADMAP.md`](ROADMAP.md)
- Product requirements: [`PRODUCT-SPEC.md`](PRODUCT-SPEC.md)
- Design rules: [`DESIGN-SYSTEM.md`](DESIGN-SYSTEM.md)
- Last status update: 2026-11-17 (Phase 4 Workbench Navigation & Layout: Command Center, Titlebar layout controls, Layout Presets [Coding, Agentic, Review, Debug, Minimal, Presentation], Reset UI Layout, and panel resize limits verified in GitHub Actions runs 44094917730 & 44095115155)

## Evidence ledger
| Area | Status | Evidence required |
|---|---|---|
| Build/toolchain | VERIFIED | Ubuntu 22.04/24.04, Rust 1.98.0, CI runs [44083923913](https://github.com/dhapidpad/Antares-Ide/actions/runs/44083923913), [44091118104](https://github.com/dhapidpad/Antares-Ide/actions/runs/44091118104) & [44095115155](https://github.com/dhapidpad/Antares-Ide/actions/runs/44095115155) |
| Full workspace check | VERIFIED | `cargo check --workspace` clean (CI Run [44095115155](https://github.com/dhapidpad/Antares-Ide/actions/runs/44095115155)) |
| Antares agent crate tests | VERIFIED | 61 unit/integration tests PASS (CI Run [44094917730](https://github.com/dhapidpad/Antares-Ide/actions/runs/44094917730) & [44095115155](https://github.com/dhapidpad/Antares-Ide/actions/runs/44095115155)) |
| Full workspace tests | VERIFIED | `cargo test --workspace` (100+ tests across `lapce-core`, `lapce-proxy`, `lapce-app`, `antares-agent`) PASS, 0 failed (CI Run [44095115155](https://github.com/dhapidpad/Antares-Ide/actions/runs/44095115155)) |
| Design tokens & themes | VERIFIED (PHASE 3) | Antares Aurora dark/light tokens, theme/icon aliases, safe svg fallback tested (CI Run [44091118104](https://github.com/dhapidpad/Antares-Ide/actions/runs/44091118104)) |
| Semantic IconRegistry | VERIFIED (PHASE 3) | Namespaces, semantic icons, default fallbacks, custom override & SVG sanitizer tested (CI Run [44091118104](https://github.com/dhapidpad/Antares-Ide/actions/runs/44091118104)) |
| Workbench navigation & layouts | VERIFIED (PHASE 4) | Layout presets (Coding, Agentic, Review, Debug, Minimal, Presentation), reset layout, titlebar layout toggles, command center and panel resize limits tested (CI Run [44095115155](https://github.com/dhapidpad/Antares-Ide/actions/runs/44095115155)) |
| Antares agent runtime | NOT VERIFIED | process lifecycle + ACP E2E test with real Octomind binary |
| Agent UI | NOT VERIFIED | desktop smoke test + real event wiring (Floem effect handling) |
| Multi-agent workflows | NOT VERIFIED | parallel/sequence/recovery tests |
| Generic MCP client | NOT VERIFIED | protocol contract tests |
| Plugin/skill manager | NOT VERIFIED | install/trust/rollback tests |
| Local API | NOT VERIFIED | auth/scope/loopback tests |
| Git UX | NOT VERIFIED | clone/push/credential tests |
| Discord RPC | NOT VERIFIED | opt-in/privacy/failure tests |
| Rebrand migration | VERIFIED (PHASE 1) | Data directory migration + non-destructive legacy import tested (CI Run [44084022073](https://github.com/dhapidpad/Antares-Ide/actions/runs/44084022073)) |
| Dead-code cleanup | NOT VERIFIED | reviewed inventory + before/after metrics |

## Rules for status updates
- Use `DONE` only with exact commit SHA and test/CI evidence.
- Use `IN PROGRESS`, `BLOCKED`, `NOT STARTED`, or `NOT VERIFIED` honestly.
- Record OS/toolchain, commands, exit codes, CI run URL, screenshots for UI work.
- Do not copy claims from old status documents without reproducing them.
