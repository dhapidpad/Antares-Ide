# Workflow: Dead-code, dependency and performance cleanup

1. Read `docs/PERFORMANCE.md` and `.agent/rules/15-docs-and-cleanup.md`.
2. Capture baseline build time, binary size, startup, idle RSS/CPU and relevant benchmark.
3. Inventory compiler warnings, TODO/stubs, dependencies, feature flags, icon assets and duplicate docs.
4. Check cfg targets, macros, dynamic registration, serialization, build scripts, public APIs and plugin references.
5. Create a candidate list with evidence and risk; do not delete by grep alone.
6. Remove one bounded batch; avoid unrelated formatting or broad refactors.
7. Run targeted tests, workspace checks, packaging checks and diff review.
8. Measure before/after and record any regression or actual gain.
9. Preserve legal/upstream/compatibility docs and fixtures.
