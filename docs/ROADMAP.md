# Antares Roadmap — Single Source of Truth

Chỉ roadmap này là nguồn thứ tự triển khai chính thức. Một phase chỉ đóng khi acceptance gate có bằng chứng.

| Phase | Scope | Gate |
|---|---|---|
| P0 | Source audit, baseline, dependency/docs inventory | Audit report + reproducible checks |
| P1 | Lapce → Antares identity & data migration | Clean install + upgrade/rollback OS matrix |
| P2 | Docs, architecture, ADR consolidation | No conflicting active spec/roadmap |
| P3 | Design tokens, typography, IconRegistry | Contrast/accessibility + component preview |
| P4 | Workbench navigation/layout/command center | Keyboard + resize + screenshot review |
| P5 | Agent runtime correctness, session, cancel, patch review | End-to-end prompt→diff→approve |
| P6 | Agents tab, multi-agent orchestration, workflow graph | Sequential/parallel/recovery tests |
| P7 | Hosted/local/custom providers and Local API | Auth/scopes/contract tests |
| P8 | MCP Hub, plugin manager, skill validation | Permission, malicious output and lifecycle tests |
| P9 | Git clone/branch/diff/commit/push/worktree | Destructive action and credential safety |
| P10 | Discord RPC and external integrations | Privacy defaults + failure isolation |
| P11 | Theme/icon/layout customization, motion, accessibility | Dark/Light/density/reduced-motion matrix |
| P12 | Dead-code/dependency/performance cleanup | Measured improvement, no regression |
| P13 | Packaging, signing, upgrade, release hardening | Cross-platform release checklist |

## Phase protocol
1. One phase/task at a time; branch `agy/<phase>-<topic>`.
2. Define acceptance criteria, files, risks, tests before edit.
3. Inspect source and pinned APIs; no speculative protocol implementation.
4. Add tests, implement one vertical slice, review full diff.
5. Run targeted checks, `make -f Makefile.antares antares-gate` when available, and `git diff --check`.
6. Update STATUS with exact evidence; upstream edits update `reference/PATCHES.md`.
7. Stop if gate fails; record blocker rather than claim completion.

## Phase checklist
Each phase must have an implementation checklist in its issue/PR. Do not duplicate a second master roadmap in a prompt or agent file.
