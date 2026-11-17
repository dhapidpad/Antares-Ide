# Workflow: Design and implement a multi-agent workflow

1. Read `docs/EXTENSIBILITY.md`, `docs/SECURITY.md` and `.agent/rules/12-agent-orchestration.md`.
2. Define one concrete user task and workflow graph; specify inputs, outputs, agent roles, provider requirements and permissions.
3. Decide whether steps are sequential, parallel, conditional, approval-gated or supervisor-managed.
4. Assign file ownership or isolated Git worktrees before parallel execution.
5. Define budgets, concurrency, retries, timeouts, cancellation and recovery semantics.
6. Validate graph schema and reject cycles, missing inputs and permission mismatches before launch.
7. Implement typed events and persistent run trace; do not share mutable context between agents.
8. Test success, partial failure, timeout, cancel, crash/restart, conflicting patches and denied permissions.
9. Ensure all writes/commands/Git operations require explicit approval.
10. Update docs and report actual tests and known limitations.
