# Antares Performance & Cleanup Policy

## Budgets (targets, not current claims)
- Command palette open: p95 ≤100 ms.
- UI interaction: p95 ≤100 ms.
- No UI-thread blocking during agent streaming.
- Idle CPU should settle near zero; no global busy polling.
- Agent/MCP/Discord processes launch only when enabled and needed.
- Record cold start, idle RSS, peak RSS on large project, binary size, build time and cancel latency.

## Runtime rules
- Lazy-start sidecars and network services.
- Bounded queues, backpressure, cancellation, timeout and output caps.
- Virtualize long transcripts, file lists and workflow canvases.
- Reuse existing editor indexing/LSP; do not duplicate services.
- Cache with TTL, size limit, eviction and invalidation.
- Avoid adding a second async runtime/UI framework without ADR and benchmark.

## Dead-code/dependency cleanup
1. Inventory TODO/FIXME/stubs, warnings, feature flags, dependencies, icons and docs.
2. Verify dynamic registration, macros, cfg targets, build scripts, serialization and public APIs.
3. Remove one bounded batch at a time.
4. Run tests and measure before/after.
5. Keep upstream, licensing, migration, compatibility, generated and fixture files unless proven obsolete.

Suggested commands: `cargo tree -d`, `cargo tree -e features`, `cargo check --workspace --all-targets`, `cargo clippy --workspace --all-targets`, `cargo test --workspace`, `rg -n 'TODO|FIXME|XXX|todo!|unimplemented!'`. Use cargo-machete/udeps/bloat only when available; verify false positives.
