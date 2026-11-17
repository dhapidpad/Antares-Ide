# Workflow: Implement a Git feature

1. Read `docs/GIT-INTEGRATION.md` and `.agent/rules/14-git-safety.md`.
2. Inspect existing Git backend and credential integration; prefer existing abstractions.
3. Specify command ID, UX entry points, exact affected refs/files, risk level and cancellation behavior.
4. Implement read-only preview before write operation where possible.
5. Require explicit confirmation for destructive or remote writes; force-push disabled by default.
6. Add tests for auth failure, network failure, non-fast-forward, conflicts, cancellation and partial completion.
7. Verify tokens never appear in URLs, logs or settings.
8. Run targeted checks and document OS/backend limitations.
