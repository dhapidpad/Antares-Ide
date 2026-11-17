# Antares Extensibility: Agents, Workflows, Skills, Providers, MCP & Plugins

## Separate extension types
- **Agent runtime:** execution process/session and event stream.
- **Agent role:** prompt, policy, model and tool set.
- **Workflow:** versioned graph of agent/tool/condition/approval steps.
- **Skill:** validated reusable instructions and resources; not arbitrary trusted code.
- **Provider adapter:** hosted/local/custom model API integration.
- **MCP server:** tools/resources/prompts exposed through the MCP protocol.
- **Antares plugin:** UI/command/language/theme contribution through a declared extension API.

## Agent roles
Orchestrator, Planner, UI/UX Designer, Implementer, Reviewer, Test Engineer, Security Auditor, Performance Engineer, Documentation Agent, Git Agent, MCP Integrator and Research Agent. Role labels do not grant OS permissions.

## Workflow engine
Must support sequential, parallel fan-out/fan-in, supervisor-worker, conditional branch, retry/backoff, human approval, sub-workflow, pause/resume/cancel and recovery. Graph validation checks cycles, missing inputs, invalid capabilities, budgets and unreachable nodes. Each run records trace, artifacts, tool calls, approvals and outcome.

## Conflict control
Parallel agents use isolated worktrees or explicit file ownership; no shared mutable working tree writes. Merge queue requires diff review and test results.

## Providers
Adapter contract: model discovery, credential validation, session start, streaming, cancel, capabilities, health, usage and close. Support local endpoints and hosted providers only after API contract verification. Secrets never appear in exported config.

## MCP and plugin trust
Capability discovery is runtime-driven. Tool permissions are per server/tool/agent. Plugins declare publisher, version, license, hash/signature, engine compatibility, dependencies and permissions. Install/update/uninstall supports rollback. Unknown protocol capabilities degrade safely.

## Local API
Versioned `/api/v1`, loopback-only, token auth, scopes (`read`, `run`, `approve`, `workspace-write`, `git-write`, `admin`), rate limits, payload limits, CORS deny-by-default and audit. Never expose arbitrary filesystem or shell endpoints.

## Skills import
Preview source, metadata, files and requested capabilities; scan for secrets, executable payloads and prompt-injection attempts; install disabled/untrusted until user confirms. Never auto-download and execute skills from third-party ecosystems.
