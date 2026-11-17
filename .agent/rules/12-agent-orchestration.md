# Multi-agent orchestration rules

- Distinguish agent role, provider/model, runtime, workflow and tool permissions.
- Use typed messages and immutable context snapshots; do not share mutable workspace state across agents.
- Parallel agents require isolated worktrees or explicit file ownership and a merge/review queue.
- Enforce concurrency, token/cost/time budgets, timeouts, cancellation propagation and bounded output.
- Human approval is required for file apply, shell execution, Git writes, plugin install and privilege escalation.
- Record run trace, agent identity, tool calls, approvals, artifacts and terminal outcome.
- Validate workflow graph before run: cycles, missing inputs, invalid capabilities, unreachable nodes and budget violations.
- Pause/resume/recovery must not replay non-idempotent side effects without confirmation.
